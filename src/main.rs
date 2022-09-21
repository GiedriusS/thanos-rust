use std::sync::Arc;
use thanos::{
    InfoRequest, InfoResponse, Label, LabelNamesRequest, LabelNamesResponse, LabelValuesRequest,
    LabelValuesResponse, SeriesRequest, SeriesResponse, StoreType, ZLabelSet,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct StoreImpl {
    pub prometheus_client: std::sync::Arc<prometheus_client::PrometheusClient>,
}
mod prometheus_client;
mod prometheus_copy;
mod thanos;

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

    type SeriesStream = ReceiverStream<Result<SeriesResponse, Status>>;

    async fn label_values(
        &self,
        request: Request<LabelValuesRequest>,
    ) -> Result<Response<LabelValuesResponse>, Status> {
        // TODO: send req to Prometheus.
        let response = LabelValuesResponse {
            values: vec!["test".to_string()],
            warnings: vec!["warning".to_string()],
            hints: None,
        };

        Ok(Response::new(response))
    }

    async fn label_names(
        &self,
        request: Request<LabelNamesRequest>,
    ) -> Result<Response<LabelNamesResponse>, Status> {
        // TODO: send req to Prometheus.
        let response = LabelNamesResponse {
            names: vec!["test".to_string()],
            warnings: vec!["warning".to_string()],
            hints: None,
        };

        Ok(Response::new(response))
    }

    async fn series(
        &self,
        request: tonic::Request<SeriesRequest>,
    ) -> Result<Response<Self::SeriesStream>, Status> {
        let (mut tx, rx) = mpsc::channel(1);

        let client = self.prometheus_client.clone();

        tokio::spawn(async move {
            client.get_status(request, tx).await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    let store = StoreImpl {
        prometheus_client: Arc::new(prometheus_client::PrometheusClient {
            url: "http://127.0.0.1:9090".to_string(),
        }),
    };

    println!("Store server listening on {}", addr);

    Server::builder()
        .add_service(StoreServer::new(store))
        .serve(addr)
        .await?;

    Ok(())
}
