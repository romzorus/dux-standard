// The controller is handling :
    // - CLI
    // - TaskList parsing
    // - HostList parsing
    // - Assignments production
    // - Results display

use cli::prelude::*;
use connection::prelude::*;
use hostparser::*;
use taskexec::prelude::*;
use taskparser::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use log::{debug, error, log_enabled, info, Level, warn};
use env_logger::Env;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments, BasicGetArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
    BasicProperties,
};
use tokio::time::Duration;
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {
    let env = Env::default()
    .filter_or("INFO", "info");

    env_logger::init_from_env(env);

    welcome_message_controller();

    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // Build a TaskList (YAML is assumed for now)
    let tasklist = tasklist_parser(
        tasklist_get_from_file(&cliargs.tasklist)
        );
    
    if tasklist.tasks.is_empty() {
        warn!("No task in given list ({})", &cliargs.tasklist);
        exit(0);
    }
    
    // Build a HostList
    let hostlist = hostlist_parser(
        hostlist_get_from_file(&cliargs.hostlist)
    );

    if hostlist.hosts.is_none() && hostlist.groups.is_none() {
        warn!("No hosts in given list ({})", &cliargs.hostlist);
        exit(0);
    }

    // Build Assignments (an Assignment is basically a Host associated to a TaskList)
    //  -> Initialization of CorrelationId (not really required for all-in-one mode)
    let mut correlationid = CorrelationIdGenerator::new();
    match correlationid.init() {
        Ok(_) => {}
        Err(e) => {
            error!("Error: failure to initialize CorrelationId : {:?}", e);
            exit(1);
        }
    }
    //  -> Actual build of Assignments
    let mut assignmentlist: Vec<Assignment> = Vec::new();
    let mut correlationidlist: Vec<String> = Vec::new();

    // This unwrap is safe since we checked before that the list is not empty.
    for host in hostlist_get_all_hosts(&hostlist).unwrap() {

        let authmode = match &cliargs.key {
            Some(privatekeypath) => {
                Ssh2AuthMode::SshKeys((
                    cliargs.user.clone(),
                    PathBuf::from(privatekeypath)
                ))
            }
            None => {
                // No SSH key given as argument, trying with password if it is given
                match cliargs.password.clone() {
                    Some(pwd) => {
                        Ssh2AuthMode::UsernamePassword(
                            Credentials::from(cliargs.user.clone(), pwd)
                        )
                    }
                    None => {
                        error!("No SSH key or password to connect to remote host.");
                        exit(1);
                    }
                }
            }
        };

        // This unwrap() is safe because initialization is checked before.
        let correlationid = correlationid.get_new_value().unwrap();
        correlationidlist.push(correlationid.clone());
        
        assignmentlist.push(Assignment::from(
            correlationid,
            RunningMode::Apply,
            host.clone(),
            ConnectionMode::Ssh2,
            authmode,
            tasklist.clone(),
            ChangeList::new(),
            TaskListResult::new(),
            AssignmentFinalStatus::Unset
        ));
    }
 
    let mut resultslist: Vec<Assignment> = Vec::new();

    // Now, Assignments need to be sent to Message Broker (MB).
    // After this, "consume" the results from MB and push them in 'resultslist'.
    // To make sure we have all results, establish a list of CorrelationId and
    // use it as a checklist.

    // let correlationIdList = ...

    // resultslist.push(assignment);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // declare a durable queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            "assignments",
        ))
        .await
        .unwrap()
        .unwrap();

    // bind the queue to exchange
    let routing_key = "assignments";
    let exchange_name = "amq.direct";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();

    // publish message  
    for assignment in assignmentlist.iter() {

        let content = serde_json::to_string(&assignment).unwrap().into_bytes();

        // create arguments for basic_publish
        let args = BasicPublishArguments::new(exchange_name, routing_key);

        channel
            .basic_publish(BasicProperties::default(), content, args)
            .await
            .unwrap();

        info!("{} : assignment sent to message broker", assignment.correlationid);
    }
    println!("");
    // Fetch a Result
    let args = BasicGetArguments::new("results")
        .no_ack(true)
        .finish();
    
    loop {
        tokio::time::sleep(Duration::from_millis(REFRESH_INTERVAL_MILLI_SECONDS)).await;

        match channel.basic_get(args.clone()).await {
            Ok(content) => {
                match content {
                    Some((_, _, raw_message)) => {
                        let assignment_result: Assignment = serde_json::from_str(&String::from_utf8_lossy(&raw_message)).unwrap();

                        match correlationidlist.iter().position(|x| (*x).eq(&assignment_result.correlationid)) {
                            Some(index) => { correlationidlist.remove(index); }
                            None => {} // TODO: handle this case : a result is in the queue but it doesn't match the assignments run this time
                        }
                        

                        info!("{} : assignment result received", assignment_result.correlationid);

                        display_output(assignment_result.clone());
                        resultslist.push(assignment_result);

                        if correlationidlist.is_empty() {
                            break;
                        }
                    }
                    None => {}
                }                        
            }
            Err(_) => {}
        }        
    }


    channel.close().await.unwrap();
    connection.close().await.unwrap();

}