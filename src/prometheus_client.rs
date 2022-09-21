use crate::prometheus_copy::{label_matcher, read_request, LabelMatcher, Query, ReadRequest};
use crate::thanos::{SeriesRequest, SeriesResponse};
use futures_util::StreamExt;
use hyper::Body;
use prost::Message;
use snap::raw::Encoder;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub async fn get_status(
        &self,
        req: tonic::Request<SeriesRequest>,
        sender: mpsc::Sender<Result<SeriesResponse, tonic::Status>>,
    ) {
        let client = reqwest::Client::new();

        let message = req.get_ref();

        let read_request = ReadRequest {
            accepted_response_types: vec![read_request::ResponseType::StreamedXorChunks as i32],
            queries: vec![Query {
                start_timestamp_ms: message.min_time,
                end_timestamp_ms: message.max_time,
                hints: None,
                matchers: vec![LabelMatcher {
                    name: "__name__".to_string(),
                    value: "go_gc_duration_seconds".to_string(),
                    r#type: label_matcher::Type::Eq as i32,
                }],
            }],
        };

        let encoded_req = read_request.encode_to_vec();
        let mut encoder = Encoder::new();
        let compressed_req = encoder.compress_vec(&encoded_req).unwrap();

        // Parse an `http::Uri`...
        let res = client
            .post(self.url.clone() + "/api/v1/read")
            .body(Body::from(compressed_req))
            .send()
            .await
            .unwrap();

        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            // decode and send it over stream
            println!("Chunk: {:?}", item.unwrap());
        }
    }
}
