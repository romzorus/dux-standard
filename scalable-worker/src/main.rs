use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments, BasicGetArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
    BasicProperties,
};
use tokio::{runtime::Builder, task::JoinHandle};
use tokio::time::Duration;
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::filter::EnvFilter;
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;

use duxcore::prelude::*;

fn main() {
    let env = Env::default()
    .filter_or("INFO", "info");

    env_logger::init_from_env(env);

    welcome_message_scalable_worker();


    // Parse the CLI arguments
    let cliargs: CliArgsScalableWorker = parse_cli_args_scalable_worker().unwrap();

    // Get the configuration
    let conf = DuxConfigScalableWorker::from(cliargs.conf).expect("Unable to determine configuration. Abort.");
    
    // Create runtime with the following principle:
    // If the number of threads to use is not specified, use 1 thread / CPU core

    let threads_number = match cliargs.threads {
        None => { std::thread::available_parallelism().unwrap().get() }
        Some(number) => { number }
    };

    let rt = Builder::new_multi_thread()
        .worker_threads(threads_number)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..(threads_number) {
        let handle = rt.spawn(assignment_handler(conf.rabbitmq.clone()));
        handles.push(handle);
    }

    for handle in handles {
        let _ = rt.block_on(handle);
    }

}


pub async fn assignment_handler(rmq_conf: RabbitMqConfig) {

    // "Consume" Assignments from the Message Broker (MB)
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let connection = Connection::open(&OpenConnectionArguments::new(
        rmq_conf.rmq_address.as_str(),
        rmq_conf.rmq_port,
        rmq_conf.rmq_username.as_str(),
        rmq_conf.rmq_password.as_str(),
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

    // declare a durable queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            "results",
        ))
        .await
        .unwrap()
        .unwrap();

    loop { // TODO : add a graceful shutdown to this loop (with Tokio Cancellation Tokens ?)
        // Fetch an Assignment
        let args = BasicGetArguments::new("assignments")
            .no_ack(true)
            .finish();
                
        let message_raw_content: Vec<u8>;
        loop {
            tokio::time::sleep(Duration::from_millis(REFRESH_INTERVAL_MILLI_SECONDS)).await;

            match channel.basic_get(args.clone()).await {
                Ok(content) => {
                    match content {
                        Some((_, _, raw_message)) => {
                            message_raw_content = raw_message;
                            break;
                        }
                        None => {}
                    }                        
                }
                Err(_) => {}
            }
        }
        let mut assignment: Assignment = serde_json::from_str(&String::from_utf8_lossy(&message_raw_content)).unwrap();

        info!("{} : Assignment received", assignment.correlationid.clone());

        let mut hosthandler = HostHandler::from(&assignment.hosthandlinginfo).unwrap();

        let _ = hosthandler.init();

        let _ = assignment.dry_run(&mut hosthandler);
        
        if let AssignmentFinalStatus::Unset = assignment.finalstatus {
                assignment.apply(&mut hosthandler);
        }

        // Send back the result to the message broker
        // bind the queue to exchange
        let routing_key = "results";
        let exchange_name = "amq.direct";
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                exchange_name,
                routing_key,
            ))
            .await
            .unwrap();
        let serialized_result = serde_json::to_string(&assignment).unwrap().into_bytes();
        let args = BasicPublishArguments::new(exchange_name, routing_key);
        channel
            .basic_publish(BasicProperties::default(), serialized_result, args)
            .await
            .unwrap();
        info!("{} : Result sent to message broker", assignment.correlationid.clone());
    }
    
    // "Produce" Assignments results (with DryRun and Results filled) and send them to the MB
    channel.close().await.unwrap();
    connection.close().await.unwrap();
}
