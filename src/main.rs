use std::sync::Arc;
use thanos::{
    InfoRequest, InfoResponse, Label, LabelNamesRequest, LabelNamesResponse, LabelValuesRequest,
    LabelValuesResponse, SeriesRequest, SeriesResponse, StoreType, ZLabelSet,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
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
            labels: vec![
                Label {
                    name: "dc".to_string(),
                    value: "hx".to_string(),
                },
                Label {
                    name: "prometheus_node_id".to_string(),
                    value: "5".to_string(),
                },
            ],
            min_time: i64::MIN,
            max_time: i64::MAX,
            store_type: StoreType::Sidecar as i32,
            label_sets: vec![ZLabelSet {
                labels: vec![
                    Label {
                        name: "dc".to_string(),
                        value: "hx".to_string(),
                    },
                    Label {
                        name: "prometheus_node_id".to_string(),
                        value: "5".to_string(),
                    },
                ],
            }],
        };

        Ok(Response::new(response))
    }

    type SeriesStream = UnboundedReceiverStream<Result<SeriesResponse, Status>>;

    async fn label_values(
        &self,
        request: Request<LabelValuesRequest>,
    ) -> Result<Response<LabelValuesResponse>, Status> {
        let client = self.prometheus_client.clone();

        let message = request.get_ref();

        let vals = client.label_values(message.label.clone()).await;

        let response = LabelValuesResponse {
            values: vals,
            warnings: vec![],
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
        let (tx, rx) = mpsc::unbounded_channel();

        let client = self.prometheus_client.clone();

        tokio::spawn(async move {
            client.remote_read(request, tx).await;
        });

        Ok(Response::new(UnboundedReceiverStream::new(rx)))
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
