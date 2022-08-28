#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSet {
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZLabelSet {
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(enumeration = "chunk::Encoding", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Chunk`.
pub mod chunk {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Encoding {
        Xor = 0,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Series {
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "2")]
    pub chunks: ::prost::alloc::vec::Vec<AggrChunk>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggrChunk {
    #[prost(int64, tag = "1")]
    pub min_time: i64,
    #[prost(int64, tag = "2")]
    pub max_time: i64,
    #[prost(message, optional, tag = "3")]
    pub raw: ::core::option::Option<Chunk>,
    #[prost(message, optional, tag = "4")]
    pub count: ::core::option::Option<Chunk>,
    #[prost(message, optional, tag = "5")]
    pub sum: ::core::option::Option<Chunk>,
    #[prost(message, optional, tag = "6")]
    pub min: ::core::option::Option<Chunk>,
    #[prost(message, optional, tag = "7")]
    pub max: ::core::option::Option<Chunk>,
    #[prost(message, optional, tag = "8")]
    pub counter: ::core::option::Option<Chunk>,
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
        /// =
        Eq = 0,
        /// !=
        Neq = 1,
        /// =~
        Re = 2,
        /// !~
        Nre = 3,
    }
}
//// PartialResponseStrategy controls partial response handling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartialResponseStrategy {
    //// WARN strategy tells server to treat any error that will related to single StoreAPI (e.g missing chunk series because of underlying
    //// storeAPI is temporarily not available) as warning which will not fail the whole query (still OK response).
    //// Server should produce those as a warnings field in response.
    Warn = 0,
    //// ABORT strategy tells server to treat any error that will related to single StoreAPI (e.g missing chunk series because of underlying
    //// storeAPI is temporarily not available) as the gRPC error that aborts the query.
    ////
    //// This is especially useful for any rule/alert evaluations on top of StoreAPI which usually does not tolerate partial
    //// errors.
    Abort = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(message, repeated, tag = "1")]
    pub timeseries: ::prost::alloc::vec::Vec<super::prometheus_copy::TimeSeries>,
    #[prost(string, tag = "2")]
    pub tenant: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub replica: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    /// Deprecated. Use label_sets instead.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(int64, tag = "2")]
    pub min_time: i64,
    #[prost(int64, tag = "3")]
    pub max_time: i64,
    #[prost(enumeration = "StoreType", tag = "4")]
    pub store_type: i32,
    /// label_sets is an unsorted list of `ZLabelSet`s.
    #[prost(message, repeated, tag = "5")]
    pub label_sets: ::prost::alloc::vec::Vec<ZLabelSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesRequest {
    #[prost(int64, tag = "1")]
    pub min_time: i64,
    #[prost(int64, tag = "2")]
    pub max_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub matchers: ::prost::alloc::vec::Vec<LabelMatcher>,
    #[prost(int64, tag = "4")]
    pub max_resolution_window: i64,
    #[prost(enumeration = "Aggr", repeated, tag = "5")]
    pub aggregates: ::prost::alloc::vec::Vec<i32>,
    /// Deprecated. Use partial_response_strategy instead.
    #[prost(bool, tag = "6")]
    pub partial_response_disabled: bool,
    /// TODO(bwplotka): Move Thanos components to use strategy instead. Including QueryAPI.
    #[prost(enumeration = "PartialResponseStrategy", tag = "7")]
    pub partial_response_strategy: i32,
    /// skip_chunks controls whether sending chunks or not in series responses.
    #[prost(bool, tag = "8")]
    pub skip_chunks: bool,
    /// hints is an opaque data structure that can be used to carry additional information.
    /// The content of this field and whether it's supported depends on the
    /// implementation of a specific store.
    #[prost(message, optional, tag = "9")]
    pub hints: ::core::option::Option<::prost_types::Any>,
    /// Query step size in milliseconds.
    /// Deprecated: Use query_hints instead.
    #[prost(int64, tag = "10")]
    pub step: i64,
    /// Range vector selector range in milliseconds.
    /// Deprecated: Use query_hints instead.
    #[prost(int64, tag = "11")]
    pub range: i64,
    /// query_hints are the hints coming from the PromQL engine when
    /// requesting a storage.SeriesSet for a given expression.
    #[prost(message, optional, tag = "12")]
    pub query_hints: ::core::option::Option<QueryHints>,
    /// shard_info is used by the querier to request a specific
    /// shard of blocks instead of entire blocks.
    #[prost(message, optional, tag = "13")]
    pub shard_info: ::core::option::Option<ShardInfo>,
}
/// Analogous to storage.SelectHints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHints {
    /// Query step size in milliseconds.
    #[prost(int64, tag = "1")]
    pub step_millis: i64,
    /// The surrounding function or aggregation.
    #[prost(message, optional, tag = "2")]
    pub func: ::core::option::Option<Func>,
    /// The grouping expression
    #[prost(message, optional, tag = "4")]
    pub grouping: ::core::option::Option<Grouping>,
    /// Range vector selector.
    #[prost(message, optional, tag = "5")]
    pub range: ::core::option::Option<Range>,
}
/// ShardInfo are the parameters used to shard series in Stores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardInfo {
    /// The index of the current shard.
    #[prost(int64, tag = "1")]
    pub shard_index: i64,
    /// The total number of shards.
    #[prost(int64, tag = "2")]
    pub total_shards: i64,
    /// Group by or without labels.
    #[prost(bool, tag = "3")]
    pub by: bool,
    /// Labels on which to partition series.
    #[prost(string, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Func {
    /// The function or aggregation name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grouping {
    /// Indicate whether it is without or by.
    #[prost(bool, tag = "1")]
    pub by: bool,
    /// List of label names used in the grouping.
    #[prost(string, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(int64, tag = "1")]
    pub millis: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeriesResponse {
    #[prost(oneof = "series_response::Result", tags = "1, 2, 3")]
    pub result: ::core::option::Option<series_response::Result>,
}
/// Nested message and enum types in `SeriesResponse`.
pub mod series_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        //// series contains 1 response series. The series labels are sorted by name.
        #[prost(message, tag = "1")]
        Series(super::Series),
        //// warning is considered an information piece in place of series for warning purposes.
        //// It is used to warn store API user about suspicious cases or partial response (if enabled).
        #[prost(string, tag = "2")]
        Warning(::prost::alloc::string::String),
        //// hints is an opaque data structure that can be used to carry additional information from
        //// the store. The content of this field and whether it's supported depends on the
        //// implementation of a specific store. It's also implementation specific if it's allowed that
        //// multiple SeriesResponse frames contain hints for a single Series() request and how should they
        //// be handled in such case (ie. merged vs keep the first/last one).
        #[prost(message, tag = "3")]
        Hints(::prost_types::Any),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelNamesRequest {
    #[prost(bool, tag = "1")]
    pub partial_response_disabled: bool,
    /// TODO(bwplotka): Move Thanos components to use strategy instead. Including QueryAPI.
    #[prost(enumeration = "PartialResponseStrategy", tag = "2")]
    pub partial_response_strategy: i32,
    #[prost(int64, tag = "3")]
    pub start: i64,
    #[prost(int64, tag = "4")]
    pub end: i64,
    /// hints is an opaque data structure that can be used to carry additional information.
    /// The content of this field and whether it's supported depends on the
    /// implementation of a specific store.
    #[prost(message, optional, tag = "5")]
    pub hints: ::core::option::Option<::prost_types::Any>,
    #[prost(message, repeated, tag = "6")]
    pub matchers: ::prost::alloc::vec::Vec<LabelMatcher>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelNamesResponse {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    //// hints is an opaque data structure that can be used to carry additional information from
    //// the store. The content of this field and whether it's supported depends on the
    //// implementation of a specific store.
    #[prost(message, optional, tag = "3")]
    pub hints: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValuesRequest {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub partial_response_disabled: bool,
    /// TODO(bwplotka): Move Thanos components to use strategy instead. Including QueryAPI.
    #[prost(enumeration = "PartialResponseStrategy", tag = "3")]
    pub partial_response_strategy: i32,
    #[prost(int64, tag = "4")]
    pub start: i64,
    #[prost(int64, tag = "5")]
    pub end: i64,
    /// hints is an opaque data structure that can be used to carry additional information.
    /// The content of this field and whether it's supported depends on the
    /// implementation of a specific store.
    #[prost(message, optional, tag = "6")]
    pub hints: ::core::option::Option<::prost_types::Any>,
    #[prost(message, repeated, tag = "7")]
    pub matchers: ::prost::alloc::vec::Vec<LabelMatcher>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValuesResponse {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    //// hints is an opaque data structure that can be used to carry additional information from
    //// the store. The content of this field and whether it's supported depends on the
    //// implementation of a specific store.
    #[prost(message, optional, tag = "3")]
    pub hints: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoreType {
    Unknown = 0,
    Query = 1,
    Rule = 2,
    Sidecar = 3,
    Store = 4,
    Receive = 5,
    /// DEBUG represents some debug StoreAPI components e.g. thanos tools store-api-serve.
    Debug = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Aggr {
    Raw = 0,
    Count = 1,
    Sum = 2,
    Min = 3,
    Max = 4,
    Counter = 5,
}
/// Generated client implementations.
pub mod store_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    //// Store represents API against instance that stores XOR encoded values with label set metadata (e.g Prometheus metrics).
    #[derive(Debug, Clone)]
    pub struct StoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StoreClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StoreClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StoreClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        //// Info returns meta information about a store e.g labels that makes that store unique as well as time range that is
        //// available.
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::InfoRequest>,
        ) -> Result<tonic::Response<super::InfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/thanos.Store/Info");
            self.inner.unary(request.into_request(), path, codec).await
        }
        //// Series streams each Series (Labels and chunk/downsampling chunk) for given label matchers and time range.
        ////
        //// Series should strictly stream full series after series, optionally split by time. This means that a single frame can contain
        //// partition of the single series, but once a new series is started to be streamed it means that no more data will
        //// be sent for previous one.
        //// Series has to be sorted.
        ////
        //// There is no requirements on chunk sorting, however it is recommended to have chunk sorted by chunk min time.
        //// This heavily optimizes the resource usage on Querier / Federated Queries.
        pub async fn series(
            &mut self,
            request: impl tonic::IntoRequest<super::SeriesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SeriesResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/thanos.Store/Series");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        //// LabelNames returns all label names constrained by the given matchers.
        pub async fn label_names(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelNamesRequest>,
        ) -> Result<tonic::Response<super::LabelNamesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/thanos.Store/LabelNames");
            self.inner.unary(request.into_request(), path, codec).await
        }
        //// LabelValues returns all label values for given label name.
        pub async fn label_values(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelValuesRequest>,
        ) -> Result<tonic::Response<super::LabelValuesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/thanos.Store/LabelValues");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod writeable_store_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    //// WriteableStore represents API against instance that stores XOR encoded values with label set metadata (e.g Prometheus metrics).
    #[derive(Debug, Clone)]
    pub struct WriteableStoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WriteableStoreClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WriteableStoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WriteableStoreClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            WriteableStoreClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// WriteRequest allows you to write metrics to this store via remote write
        pub async fn remote_write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/thanos.WriteableStore/RemoteWrite");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod store_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with StoreServer.
    #[async_trait]
    pub trait Store: Send + Sync + 'static {
        //// Info returns meta information about a store e.g labels that makes that store unique as well as time range that is
        //// available.
        async fn info(
            &self,
            request: tonic::Request<super::InfoRequest>,
        ) -> Result<tonic::Response<super::InfoResponse>, tonic::Status>;
        ///Server streaming response type for the Series method.
        type SeriesStream: futures_core::Stream<Item = Result<super::SeriesResponse, tonic::Status>>
            + Send
            + 'static;
        //// Series streams each Series (Labels and chunk/downsampling chunk) for given label matchers and time range.
        ////
        //// Series should strictly stream full series after series, optionally split by time. This means that a single frame can contain
        //// partition of the single series, but once a new series is started to be streamed it means that no more data will
        //// be sent for previous one.
        //// Series has to be sorted.
        ////
        //// There is no requirements on chunk sorting, however it is recommended to have chunk sorted by chunk min time.
        //// This heavily optimizes the resource usage on Querier / Federated Queries.
        async fn series(
            &self,
            request: tonic::Request<super::SeriesRequest>,
        ) -> Result<tonic::Response<Self::SeriesStream>, tonic::Status>;
        //// LabelNames returns all label names constrained by the given matchers.
        async fn label_names(
            &self,
            request: tonic::Request<super::LabelNamesRequest>,
        ) -> Result<tonic::Response<super::LabelNamesResponse>, tonic::Status>;
        //// LabelValues returns all label values for given label name.
        async fn label_values(
            &self,
            request: tonic::Request<super::LabelValuesRequest>,
        ) -> Result<tonic::Response<super::LabelValuesResponse>, tonic::Status>;
    }
    //// Store represents API against instance that stores XOR encoded values with label set metadata (e.g Prometheus metrics).
    #[derive(Debug)]
    pub struct StoreServer<T: Store> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Store> StoreServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StoreServer<T>
    where
        T: Store,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/thanos.Store/Info" => {
                    #[allow(non_camel_case_types)]
                    struct InfoSvc<T: Store>(pub Arc<T>);
                    impl<T: Store> tonic::server::UnaryService<super::InfoRequest> for InfoSvc<T> {
                        type Response = super::InfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/thanos.Store/Series" => {
                    #[allow(non_camel_case_types)]
                    struct SeriesSvc<T: Store>(pub Arc<T>);
                    impl<T: Store> tonic::server::ServerStreamingService<super::SeriesRequest> for SeriesSvc<T> {
                        type Response = super::SeriesResponse;
                        type ResponseStream = T::SeriesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SeriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).series(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SeriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/thanos.Store/LabelNames" => {
                    #[allow(non_camel_case_types)]
                    struct LabelNamesSvc<T: Store>(pub Arc<T>);
                    impl<T: Store> tonic::server::UnaryService<super::LabelNamesRequest> for LabelNamesSvc<T> {
                        type Response = super::LabelNamesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LabelNamesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).label_names(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LabelNamesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/thanos.Store/LabelValues" => {
                    #[allow(non_camel_case_types)]
                    struct LabelValuesSvc<T: Store>(pub Arc<T>);
                    impl<T: Store> tonic::server::UnaryService<super::LabelValuesRequest> for LabelValuesSvc<T> {
                        type Response = super::LabelValuesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LabelValuesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).label_values(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LabelValuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Store> Clone for StoreServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Store> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Store> tonic::transport::NamedService for StoreServer<T> {
        const NAME: &'static str = "thanos.Store";
    }
}
/// Generated server implementations.
pub mod writeable_store_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with WriteableStoreServer.
    #[async_trait]
    pub trait WriteableStore: Send + Sync + 'static {
        /// WriteRequest allows you to write metrics to this store via remote write
        async fn remote_write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status>;
    }
    //// WriteableStore represents API against instance that stores XOR encoded values with label set metadata (e.g Prometheus metrics).
    #[derive(Debug)]
    pub struct WriteableStoreServer<T: WriteableStore> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WriteableStore> WriteableStoreServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WriteableStoreServer<T>
    where
        T: WriteableStore,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/thanos.WriteableStore/RemoteWrite" => {
                    #[allow(non_camel_case_types)]
                    struct RemoteWriteSvc<T: WriteableStore>(pub Arc<T>);
                    impl<T: WriteableStore> tonic::server::UnaryService<super::WriteRequest> for RemoteWriteSvc<T> {
                        type Response = super::WriteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remote_write(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoteWriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: WriteableStore> Clone for WriteableStoreServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WriteableStore> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WriteableStore> tonic::transport::NamedService for WriteableStoreServer<T> {
        const NAME: &'static str = "thanos.WriteableStore";
    }
}
