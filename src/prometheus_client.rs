use crate::prometheus_copy::{read_request, ChunkedReadResponse, LabelMatcher, Query, ReadRequest};
use crate::thanos::{
    series_response, AggrChunk, Chunk, LabelMatcher as ThanosLabelMatcher, Series, SeriesRequest,
    SeriesResponse,
};
use bytes::Buf;
use bytes::BytesMut;
use crc::{Crc, CRC_32_ISCSI};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use hyper::Body;
use prost::Message;
use serde::Deserialize;
use snap::raw::Encoder;
use std::io::Read;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_util::io::StreamReader;

#[derive(Default)]
pub struct PrometheusClient {
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct LabelValuesPromResp {
    data: Vec<String>,
}

fn convert_matcher(x: &ThanosLabelMatcher) -> LabelMatcher {
    LabelMatcher {
        r#type: x.r#type,
        name: x.name.clone(),
        value: x.value.clone(),
    }
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

fn thanos_to_prom_matchers(thanos_matchers: &Vec<ThanosLabelMatcher>) -> Vec<LabelMatcher> {
    thanos_matchers.iter().map(convert_matcher).collect()
}

impl PrometheusClient {
    pub async fn label_values(&self, label: String) -> Vec<String> {
        let res = reqwest::get(self.url.clone() + "/api/v1/label/" + &label + "/values")
            .await
            .unwrap();

        let resp = res.json::<LabelValuesPromResp>().await.unwrap();

        resp.data
    }

    pub async fn remote_read(
        &self,
        req: tonic::Request<SeriesRequest>,
        sender: mpsc::Sender<Result<SeriesResponse, tonic::Status>>,
    ) {
        let client = reqwest::Client::builder().http1_only().build().unwrap();

        let message = req.get_ref();
        let read_request = ReadRequest {
            accepted_response_types: vec![read_request::ResponseType::StreamedXorChunks as i32],
            queries: vec![Query {
                start_timestamp_ms: message.min_time,
                end_timestamp_ms: message.max_time,
                hints: None,
                matchers: thanos_to_prom_matchers(&message.matchers),
            }],
        };

        let encoded_req = read_request.encode_to_vec();
        let mut encoder = Encoder::new();
        let compressed_req = encoder.compress_vec(&encoded_req).unwrap();

        let mut res = client
            .post(self.url.clone() + "/api/v1/read")
            .body(Body::from(compressed_req))
            .send()
            .await
            .unwrap();

        print!("response headers: {:?}\n", res.headers());

        let mut stream = res.bytes_stream();
        let mut stream_reader = StreamReader::new(
            stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
        );

        let mut read_buffer: Box<Vec<u8>> = Box::new(vec![]);

        loop {
            let mut crc: [u8; 4] = [0, 0, 0, 0];

            let data_size_result = unsigned_varint::aio::read_usize(&mut stream_reader).await;
            let data_size = match data_size_result {
                Ok(v) => v,
                Err(_) => break,
            };
            stream_reader.read_exact(&mut crc).await.unwrap();

            read_buffer.resize(data_size, 0);
            if let Err(e) = stream_reader.read_exact(&mut read_buffer).await {
                print!("err occurred {:?}!\n", e);
                continue;
            }

            pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

            let calculated_checksum = CASTAGNOLI.checksum(&read_buffer);

            if as_u32_be(&crc) != calculated_checksum {
                panic!(
                    "invalid CRC32: got {}, received {}",
                    as_u32_be(&crc),
                    calculated_checksum
                );
            }

            let resp = ChunkedReadResponse::decode(BytesMut::from(read_buffer.as_slice())).unwrap();

            for chunked_series in resp.chunked_series {
                let mut thanos_chks: Vec<AggrChunk> = vec![];

                for prom_chk in chunked_series.chunks {
                    thanos_chks.push(AggrChunk {
                        min_time: prom_chk.min_time_ms,
                        max_time: prom_chk.max_time_ms,
                        raw: Some(Chunk {
                            data: prom_chk.data,
                            r#type: prom_chk.r#type - 1,
                        }),
                        count: None,
                        counter: None,
                        max: None,
                        min: None,
                        sum: None,
                    });
                }
                // TODO: Extend with external labelset.

                let series_resp = SeriesResponse {
                    result: Some(series_response::Result::Series(Series {
                        labels: chunked_series.labels,
                        chunks: thanos_chks,
                    })),
                };

                sender.send(Ok(series_resp)).await.unwrap();
            }
        }
    }
}
