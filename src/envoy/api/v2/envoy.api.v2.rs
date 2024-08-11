// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryRequest {
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<core::Node>,
    #[prost(string, repeated, tag = "3")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub error_detail: ::core::option::Option<super::super::super::google::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryResponse {
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<super::super::super::google::protobuf::Any>,
    #[prost(bool, tag = "3")]
    pub canary: bool,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub control_plane: ::core::option::Option<core::ControlPlane>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryRequest {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<core::Node>,
    #[prost(string, tag = "2")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub resource_names_subscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "4")]
    pub resource_names_unsubscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "5")]
    pub initial_resource_versions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "6")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub error_detail: ::core::option::Option<super::super::super::google::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryResponse {
    #[prost(string, tag = "1")]
    pub system_version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub removed_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRouteConfiguration {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub route_configuration_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub key: ::core::option::Option<scoped_route_configuration::Key>,
}
/// Nested message and enum types in `ScopedRouteConfiguration`.
pub mod scoped_route_configuration {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Key {
        #[prost(message, repeated, tag = "1")]
        pub fragments: ::prost::alloc::vec::Vec<key::Fragment>,
    }
    /// Nested message and enum types in `Key`.
    pub mod key {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Fragment {
            #[prost(oneof = "fragment::Type", tags = "1")]
            pub r#type: ::core::option::Option<fragment::Type>,
        }
        /// Nested message and enum types in `Fragment`.
        pub mod fragment {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                #[prost(string, tag = "1")]
                StringKey(::prost::alloc::string::String),
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SrdsDummy {}
/// Generated client implementations.
pub mod scoped_routes_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Scoped Routes Discovery Service (SRDS) API distributes
    /// :ref:`ScopedRouteConfiguration<envoy_api_msg.ScopedRouteConfiguration>`
    /// resources. Each ScopedRouteConfiguration resource represents a "routing
    /// scope" containing a mapping that allows the HTTP connection manager to
    /// dynamically assign a routing table (specified via a
    /// :ref:`RouteConfiguration<envoy_api_msg_RouteConfiguration>` message) to each
    /// HTTP request.
    #[derive(Debug, Clone)]
    pub struct ScopedRoutesDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ScopedRoutesDiscoveryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ScopedRoutesDiscoveryServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ScopedRoutesDiscoveryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ScopedRoutesDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn stream_scoped_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.api.v2.ScopedRoutesDiscoveryService/StreamScopedRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.api.v2.ScopedRoutesDiscoveryService",
                        "StreamScopedRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delta_scoped_routes(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeltaDiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.api.v2.ScopedRoutesDiscoveryService/DeltaScopedRoutes",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.api.v2.ScopedRoutesDiscoveryService",
                        "DeltaScopedRoutes",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn fetch_scoped_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscoveryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.api.v2.ScopedRoutesDiscoveryService/FetchScopedRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.api.v2.ScopedRoutesDiscoveryService",
                        "FetchScopedRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scoped_routes_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ScopedRoutesDiscoveryServiceServer.
    #[async_trait]
    pub trait ScopedRoutesDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamScopedRoutes method.
        type StreamScopedRoutesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_scoped_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamScopedRoutesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaScopedRoutes method.
        type DeltaScopedRoutesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_scoped_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaScopedRoutesStream>,
            tonic::Status,
        >;
        async fn fetch_scoped_routes(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    /// The Scoped Routes Discovery Service (SRDS) API distributes
    /// :ref:`ScopedRouteConfiguration<envoy_api_msg.ScopedRouteConfiguration>`
    /// resources. Each ScopedRouteConfiguration resource represents a "routing
    /// scope" containing a mapping that allows the HTTP connection manager to
    /// dynamically assign a routing table (specified via a
    /// :ref:`RouteConfiguration<envoy_api_msg_RouteConfiguration>` message) to each
    /// HTTP request.
    #[derive(Debug)]
    pub struct ScopedRoutesDiscoveryServiceServer<T: ScopedRoutesDiscoveryService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: ScopedRoutesDiscoveryService> ScopedRoutesDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ScopedRoutesDiscoveryServiceServer<T>
    where
        T: ScopedRoutesDiscoveryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/envoy.api.v2.ScopedRoutesDiscoveryService/StreamScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct StreamScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamScopedRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamScopedRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ScopedRoutesDiscoveryService>::stream_scoped_routes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StreamScopedRoutesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.api.v2.ScopedRoutesDiscoveryService/DeltaScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaScopedRoutesSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaScopedRoutesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DeltaDiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ScopedRoutesDiscoveryService>::delta_scoped_routes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeltaScopedRoutesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.api.v2.ScopedRoutesDiscoveryService/FetchScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct FetchScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchScopedRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ScopedRoutesDiscoveryService>::fetch_scoped_routes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = FetchScopedRoutesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ScopedRoutesDiscoveryService> Clone
    for ScopedRoutesDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ScopedRoutesDiscoveryService> tonic::server::NamedService
    for ScopedRoutesDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.ScopedRoutesDiscoveryService";
    }
}
