use hyper::body::HttpBody as _;
use hyper::{Body, Client, Request};
use prometheus_copy::{label_matcher, read_request, LabelMatcher, Query, ReadRequest};
use prost::Message;
use snap::raw::Encoder;
use tokio::io::{stdout, AsyncWriteExt as _};

mod thanos {
    include!("thanos.rs");
}

mod prometheus_copy {
    include!("prometheus_copy.rs");
}

#[derive(Default)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub async fn get_status(&self) {
        println!("hello!");

        // Still inside `async fn main`...
        let client = Client::new();

        let readRequest = ReadRequest {
            accepted_response_types: vec![read_request::ResponseType::StreamedXorChunks as i32],
            queries: vec![Query {
                start_timestamp_ms: 0,
                end_timestamp_ms: 2663243063000,
                hints: None,
                matchers: vec![LabelMatcher {
                    name: "__name__".to_string(),
                    value: "go_gc_duration_seconds".to_string(),
                    r#type: label_matcher::Type::Eq as i32,
                }],
            }],
        };

        let encodedReq = readRequest.encode_to_vec();
        let mut encoder = Encoder::new();
        let compressedReq = encoder.compress_vec(&encodedReq).unwrap();

        // Parse an `http::Uri`...
        let req = Request::builder()
            .method("POST")
            .uri(self.url.clone() + "/api/v1/read")
            .body(Body::from(compressedReq))
            .unwrap();

        // Await the response...
        let mut resp = client.request(req).await.unwrap();

        while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk.unwrap()).await.unwrap();
        }

        println!("Response: {}", resp.status());
    }
}
