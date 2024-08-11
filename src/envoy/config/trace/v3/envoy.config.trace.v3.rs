// This file is @generated by prost-build.
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
