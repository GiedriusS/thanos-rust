use crate::prometheus_copy::{read_request, LabelMatcher, Query, ReadRequest};
use crate::thanos::{LabelMatcher as ThanosLabelMatcher, SeriesRequest, SeriesResponse};
use bytes::Buf;
use crc::{Algorithm, Crc, CRC_32_ISCSI};
use futures_util::StreamExt;
use hyper::Body;
use prost::Message;
use snap::raw::Encoder;
use std::io::Read;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct PrometheusClient {
    pub url: String,
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

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

fn thanos_to_prom_matchers(thanos_matchers: &Vec<ThanosLabelMatcher>) -> Vec<LabelMatcher> {
    thanos_matchers.iter().map(convert_matcher).collect()
}

impl PrometheusClient {
    pub async fn get_status(
        &self,
        req: tonic::Request<SeriesRequest>,
        sender: mpsc::Sender<Result<SeriesResponse, tonic::Status>>,
    ) {
        let client = reqwest::Client::new();

        let message = req.get_ref();

        // TODO: stream responses and send to client.

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

        let res = client
            .post(self.url.clone() + "/api/v1/read")
            .body(Body::from(compressed_req))
            .send()
            .await
            .unwrap();

        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            // decode and send it over stream
            // uvarint -> size of data portion ()
            // big endian crc32 (4B)
            // data
            // compare crc32
            let data = item.unwrap();

            let mut reader = data.reader();

            let mut crc: [u8; 4] = [0, 0, 0, 0];

            let data_size = unsigned_varint::io::read_usize(&mut reader).unwrap();
            print!("data size is {}", data_size);

            reader.read_exact(&mut crc).unwrap();

            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();

            pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

            let calculated_checksum = CASTAGNOLI.checksum(&data);

            print!(
                "calculated crc32 {}, got crc32 {}, vec len {}, got data length {}\n",
                calculated_checksum,
                as_u32_be(&crc),
                data.len(),
                data_size,
            );
        }
    }
}
