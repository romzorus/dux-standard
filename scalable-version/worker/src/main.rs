use cli::prelude::*;
use connection::prelude::*;
use std::sync::Mutex;
use taskexec::prelude::*;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments, BasicGetArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
    BasicProperties,
};

use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {

    welcome_message();
    println!("[Dux worker]"); // TODO : have a nice display for this also
    println!("");

    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // "Consume" Assignments from the Message Broker (MB)
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

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let args = BasicGetArguments::new("assignments")
        .no_ack(true)
        .finish();
        
    let (_, _, message_raw_content) = channel
        .basic_get(args)
        .await
        .unwrap()
        .unwrap();

    let assignment: Assignment = serde_json::from_str(&String::from_utf8_lossy(&message_raw_content)).unwrap();

    println!("Assignment received :");
    println!("{:#?}", assignment);

    channel.close().await.unwrap();
    connection.close().await.unwrap();

    // let mut assignmentlist: Vec<Assignment> = Vec::new();

    // // If the number of threads to use is not specified, one thread per CPU of the local machine
    // let threads_number = match cliargs.threads {
    //     None => { std::thread::available_parallelism().unwrap().get() }
    //     Some(number) => { number }
    // };
 
    // let resultslist: Mutex<Vec<Assignment>> = Mutex::new(Vec::new());

    // let pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(threads_number)
    //     .build()
    //     .unwrap();

    // pool.install(|| {
    //     rayon::scope(|s| {
    //         for mut assignment in assignmentlist.into_iter() {
    //             let resultslist = &resultslist;
    //             s.spawn(move |_| {

    //                 let mut hosthandler = HostHandler::from(
    //                     assignment.connectionmode.clone(),
    //                     assignment.host.clone(),
    //                     assignment.authmode.clone()
    //                 );

    //                 let _ = hosthandler.init();

    //                 let _ = assignment.dry_run(&mut hosthandler);
    //                 if let AssignmentFinalStatus::Unset = assignment.finalstatus {
    //                         assignment.apply(&mut hosthandler);
    //                 }
    //                 resultslist.lock().unwrap().push(assignment);
    //             });
    //         }
    //     });
    // });
    
    // "Produce" Assignments results (with DryRun and Results filled) and send them to the MB

}