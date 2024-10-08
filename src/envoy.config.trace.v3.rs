// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tracing {
    #[prost(message, optional, tag = "1")]
    pub http: ::core::option::Option<tracing::Http>,
}
/// Nested message and enum types in `Tracing`.
pub mod tracing {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "http::ConfigType", tags = "3")]
        pub config_type: ::core::option::Option<http::ConfigType>,
    }
    /// Nested message and enum types in `Http`.
    pub mod http {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DatadogRemoteConfig {
    #[prost(message, optional, tag = "1")]
    pub polling_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatadogConfig {
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub service_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collector_hostname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub remote_config: ::core::option::Option<DatadogRemoteConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicOtConfig {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub library: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightstepConfig {
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "2")]
    pub access_token_file: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub access_token: ::core::option::Option<super::super::core::v3::DataSource>,
    #[prost(
        enumeration = "lightstep_config::PropagationMode",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub propagation_modes: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `LightstepConfig`.
pub mod lightstep_config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PropagationMode {
        Envoy = 0,
        Lightstep = 1,
        B3 = 2,
        TraceContext = 3,
    }
    impl PropagationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PropagationMode::Envoy => "ENVOY",
                PropagationMode::Lightstep => "LIGHTSTEP",
                PropagationMode::B3 => "B3",
                PropagationMode::TraceContext => "TRACE_CONTEXT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENVOY" => Some(Self::Envoy),
                "LIGHTSTEP" => Some(Self::Lightstep),
                "B3" => Some(Self::B3),
                "TRACE_CONTEXT" => Some(Self::TraceContext),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenCensusConfig {
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub trace_config: ::core::option::Option<
        super::super::super::super::opencensus::proto::trace::v1::TraceConfig,
    >,
    #[deprecated]
    #[prost(bool, tag = "2")]
    pub stdout_exporter_enabled: bool,
    #[deprecated]
    #[prost(bool, tag = "3")]
    pub stackdriver_exporter_enabled: bool,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub stackdriver_project_id: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "10")]
    pub stackdriver_address: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(message, optional, tag = "13")]
    pub stackdriver_grpc_service: ::core::option::Option<
        super::super::core::v3::GrpcService,
    >,
    #[deprecated]
    #[prost(bool, tag = "5")]
    pub zipkin_exporter_enabled: bool,
    #[deprecated]
    #[prost(string, tag = "6")]
    pub zipkin_url: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(bool, tag = "11")]
    pub ocagent_exporter_enabled: bool,
    #[deprecated]
    #[prost(string, tag = "12")]
    pub ocagent_address: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(message, optional, tag = "14")]
    pub ocagent_grpc_service: ::core::option::Option<
        super::super::core::v3::GrpcService,
    >,
    #[deprecated]
    #[prost(
        enumeration = "open_census_config::TraceContext",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub incoming_trace_context: ::prost::alloc::vec::Vec<i32>,
    #[deprecated]
    #[prost(
        enumeration = "open_census_config::TraceContext",
        repeated,
        packed = "false",
        tag = "9"
    )]
    pub outgoing_trace_context: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `OpenCensusConfig`.
pub mod open_census_config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TraceContext {
        None = 0,
        TraceContext = 1,
        GrpcTraceBin = 2,
        CloudTraceContext = 3,
        B3 = 4,
    }
    impl TraceContext {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TraceContext::None => "NONE",
                TraceContext::TraceContext => "TRACE_CONTEXT",
                TraceContext::GrpcTraceBin => "GRPC_TRACE_BIN",
                TraceContext::CloudTraceContext => "CLOUD_TRACE_CONTEXT",
                TraceContext::B3 => "B3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "TRACE_CONTEXT" => Some(Self::TraceContext),
                "GRPC_TRACE_BIN" => Some(Self::GrpcTraceBin),
                "CLOUD_TRACE_CONTEXT" => Some(Self::CloudTraceContext),
                "B3" => Some(Self::B3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenTelemetryConfig {
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    #[prost(message, optional, tag = "3")]
    pub http_service: ::core::option::Option<super::super::core::v3::HttpService>,
    #[prost(string, tag = "2")]
    pub service_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub resource_detectors: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "5")]
    pub sampler: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceServiceConfig {
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkyWalkingConfig {
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    #[prost(message, optional, tag = "2")]
    pub client_config: ::core::option::Option<ClientConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConfig {
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub max_cache_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(oneof = "client_config::BackendTokenSpecifier", tags = "3")]
    pub backend_token_specifier: ::core::option::Option<
        client_config::BackendTokenSpecifier,
    >,
}
/// Nested message and enum types in `ClientConfig`.
pub mod client_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackendTokenSpecifier {
        #[prost(string, tag = "3")]
        BackendToken(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZipkinConfig {
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collector_endpoint: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub trace_id_128bit: bool,
    #[prost(message, optional, tag = "4")]
    pub shared_span_context: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(enumeration = "zipkin_config::CollectorEndpointVersion", tag = "5")]
    pub collector_endpoint_version: i32,
    #[prost(string, tag = "6")]
    pub collector_hostname: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(bool, tag = "7")]
    pub split_spans_for_request: bool,
}
/// Nested message and enum types in `ZipkinConfig`.
pub mod zipkin_config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CollectorEndpointVersion {
        DeprecatedAndUnavailableDoNotUse = 0,
        HttpJson = 1,
        HttpProto = 2,
        Grpc = 3,
    }
    impl CollectorEndpointVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse => {
                    "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE"
                }
                CollectorEndpointVersion::HttpJson => "HTTP_JSON",
                CollectorEndpointVersion::HttpProto => "HTTP_PROTO",
                CollectorEndpointVersion::Grpc => "GRPC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE" => {
                    Some(Self::DeprecatedAndUnavailableDoNotUse)
                }
                "HTTP_JSON" => Some(Self::HttpJson),
                "HTTP_PROTO" => Some(Self::HttpProto),
                "GRPC" => Some(Self::Grpc),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XRayConfig {
    #[prost(message, optional, tag = "1")]
    pub daemon_endpoint: ::core::option::Option<super::super::core::v3::SocketAddress>,
    #[prost(string, tag = "2")]
    pub segment_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub sampling_rule_manifest: ::core::option::Option<
        super::super::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "4")]
    pub segment_fields: ::core::option::Option<x_ray_config::SegmentFields>,
}
/// Nested message and enum types in `XRayConfig`.
pub mod x_ray_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SegmentFields {
        #[prost(string, tag = "1")]
        pub origin: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub aws: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Struct,
        >,
    }
}
