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

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
         BasicPublishArguments, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    
    BasicProperties,
};

use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::filter::EnvFilter;
use serde::Serialize;

#[tokio::main]
async fn main() {
    welcome_message();
    println!("[Dux controller]"); // TODO : have a nice display for this also
    println!("");

    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // Build a TaskList (YAML is assumed for now)
    let tasklist = tasklist_parser(
        tasklist_get_from_file(&cliargs.tasklist)
        );
    
    // Build a HostList (not implemented yet)
    let hostlist = hostlist_parser(
        hostlist_get_from_file(&cliargs.hostlist)
    );

    // Build Assignments (an Assignment is basically a Host associated to a TaskList)
    //  -> Initialization of CorrelationId (not really required for all-in-one mode)
    let mut correlationid = CorrelationIdGenerator::new();
    correlationid.init();
    //  -> Actual build of Assignments
    let mut assignmentlist: Vec<Assignment> = Vec::new();

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
                        panic!("No SSH key or password to connect to remote host."); // TODO : gracefully quit instead of panic
                    }
                }
            }
        };
        
        assignmentlist.push(Assignment::from(
            correlationid.get_new_value().unwrap(),
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
 
    let resultslist: Vec<Assignment> = Vec::new();

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
    }

    channel.close().await.unwrap();
    connection.close().await.unwrap();

    // TODO : implement a better way to sort the output according to the order of the hosts in the HostList
    // aka sort resultslist in HostList order so we simply have to go through resultslist after that
    for host in hostlist_get_all_hosts(&hostlist).unwrap() {
        for assignment in resultslist.clone().into_iter() {
            if host.eq(&assignment.host) {
                display_output(assignment);
            }
        }
    }
}