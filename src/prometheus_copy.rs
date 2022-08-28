#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricMetadata {
    /// Represents the metric type, these match the set from Prometheus.
    /// Refer to pkg/textparse/interface.go for details.
    #[prost(enumeration = "metric_metadata::MetricType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub metric_family_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub help: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MetricMetadata`.
pub mod metric_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MetricType {
        Unknown = 0,
        Counter = 1,
        Gauge = 2,
        Histogram = 3,
        Gaugehistogram = 4,
        Summary = 5,
        Info = 6,
        Stateset = 7,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sample {
    #[prost(double, tag = "1")]
    pub value: f64,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exemplar {
    /// Optional, can be empty.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<super::thanos::Label>,
    #[prost(double, tag = "2")]
    pub value: f64,
    /// timestamp is in ms format, see pkg/timestamp/timestamp.go for
    /// conversion from time.Time to Prometheus timestamp.
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
/// TimeSeries represents samples and labels for a single time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// Labels have to be sorted by label names and without duplicated label names.
    /// TODO(bwplotka): Don't use zero copy ZLabels, see <https://github.com/thanos-io/thanos/pull/3279> for details.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<super::thanos::Label>,
    #[prost(message, repeated, tag = "2")]
    pub samples: ::prost::alloc::vec::Vec<Sample>,
    #[prost(message, repeated, tag = "3")]
    pub exemplars: ::prost::alloc::vec::Vec<Exemplar>,
}
/// Matcher specifies a rule, which can match or set of labels or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelMatcher {
    #[prost(enumeration = "label_matcher::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LabelMatcher`.
pub mod label_matcher {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Eq = 0,
        Neq = 1,
        Re = 2,
        Nre = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadHints {
    /// Query step size in milliseconds.
    #[prost(int64, tag = "1")]
    pub step_ms: i64,
    /// String representation of surrounding function or aggregation.
    #[prost(string, tag = "2")]
    pub func: ::prost::alloc::string::String,
    /// Start time in milliseconds.
    #[prost(int64, tag = "3")]
    pub start_ms: i64,
    /// End time in milliseconds.
    #[prost(int64, tag = "4")]
    pub end_ms: i64,
    /// List of label names used in aggregation.
    #[prost(string, repeated, tag = "5")]
    pub grouping: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indicate whether it is without or by.
    #[prost(bool, tag = "6")]
    pub by: bool,
    /// Range vector selector range in milliseconds.
    #[prost(int64, tag = "7")]
    pub range_ms: i64,
}
/// Chunk represents a TSDB chunk.
/// Time range [min, max] is inclusive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(int64, tag = "1")]
    pub min_time_ms: i64,
    #[prost(int64, tag = "2")]
    pub max_time_ms: i64,
    #[prost(enumeration = "chunk::Encoding", tag = "3")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Chunk`.
pub mod chunk {
    /// We require this to match chunkenc.Encoding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Encoding {
        Unknown = 0,
        Xor = 1,
    }
}
/// ChunkedSeries represents single, encoded time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkedSeries {
    /// Labels should be sorted.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<super::thanos::Label>,
    /// Chunks will be in start time order and may overlap.
    #[prost(message, repeated, tag = "2")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
