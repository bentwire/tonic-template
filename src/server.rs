use tokio::sync::Mutex;

use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing_subscriber::prelude::*;


use {{crate_name}}::proto::rpc::{PingRequest, PingReply};
use {{crate_name}}::proto::rpc::simple_server::{Simple, SimpleServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tokio-console and tracing.
    let console_layer = console_subscriber::spawn();
    //let format_layer = tracing_subscriber::fmt::layer().pretty().with_file(true).with_level(true).with_line_number(true).with_thread_ids(true).with_thread_names(true);
    tracing_subscriber::registry()
        .with(console_layer)
        //.with(format_layer)
        .init();
    
    let addr = "0.0.0.0:50051".parse().unwrap();

    let app = SimpleService::new();

    Server::builder()
        .add_service(SimpleServer::new(app))
        .serve(addr)
        .await?;
    
    Ok(())
}

#[derive(Default)]
pub struct SimpleService {
    ping_count: Mutex<u64>,
}

impl SimpleService {
    pub fn new() -> Self {
        SimpleService { ..Default::default() }
    }
}

#[tonic::async_trait]
impl Simple for SimpleService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        tracing::info!("Got a request from: {:?}", request.remote_addr());
        let mut ping_count = self.ping_count.lock().await;
        let req = request.get_ref();
        let rep = PingReply { id: *ping_count, rid: req.id, data: req.data.clone() };
        tracing::info!("Rquest ID: {:?}", req.id);
        tracing::info!("Request data: {:?}", req.data);

        *ping_count += 1;

        Ok(Response::new(rep))
    }
}