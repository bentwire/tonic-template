use tracing_subscriber::prelude::*;

use {{crate_name}}::proto::rpc::simple_client::SimpleClient;
use {{crate_name}}::proto::rpc::PingRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tokio-console and tracing.
    //let console_layer = console_subscriber::spawn();
    let format_layer = tracing_subscriber::fmt::layer().pretty().with_file(true).with_level(true).with_line_number(true).with_thread_ids(true).with_thread_names(true);
    tracing_subscriber::registry()
        //.with(console_layer)
        .with(format_layer)
        .with(tracing::level_filters::LevelFilter::INFO)
        .init();
    
    let addr = "tcp://0.0.0.0:50051";
    let mut client = SimpleClient::connect(addr).await?;

    let mut id: u64 = 0;
    loop {
        let req = tonic::Request::new(PingRequest {
            id: id,
            data: Some("HELLO!".to_string()),
        });
    
        let res = client.ping(req).await?;
    
        tracing::info!("RES: {:?}", res);
        
        assert!(res.into_inner().rid == id);

        id += 1;
    }

    //Ok(())
}
