use thanos::{InfoRequest, InfoResponse, Label, StoreType, ZLabelSet};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct StoreImpl {}

mod thanos {
    include!("thanos.rs");
}

mod prometheus_copy {
    include!("prometheus_copy.rs");
}

use crate::thanos::store_server::Store;
use crate::thanos::store_server::StoreServer;

#[tonic::async_trait]
impl Store for StoreImpl {
    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = InfoResponse {
            labels: vec![Label {
                name: "foo".to_string(),
                value: "bar".to_string(),
            }],
            min_time: 123,
            max_time: 555,
            store_type: StoreType::Sidecar as i32,
            label_sets: vec![ZLabelSet {
                labels: vec![Label {
                    name: "foo".to_string(),
                    value: "bar".to_string(),
                }],
            }],
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let store = StoreImpl::default();

    println!("Store server listening on {}", addr);

    Server::builder()
        .add_service(StoreServer::new(store))
        .serve(addr)
        .await?;

    Ok(())
}
