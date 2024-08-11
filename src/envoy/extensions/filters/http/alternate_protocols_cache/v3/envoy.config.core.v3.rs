// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedExtensionConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TcpProtocolOptions {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QuicKeepAliveSettings {
    #[prost(message, optional, tag = "1")]
    pub max_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub initial_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuicProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub max_concurrent_streams: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "2")]
    pub initial_stream_window_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub initial_connection_window_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "4")]
    pub num_timeouts_to_trigger_port_migration: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "5")]
    pub connection_keepalive: ::core::option::Option<QuicKeepAliveSettings>,
    #[prost(string, tag = "6")]
    pub connection_options: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub client_connection_options: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub idle_network_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamHttpProtocolOptions {
    #[prost(bool, tag = "1")]
    pub auto_sni: bool,
    #[prost(bool, tag = "2")]
    pub auto_san_validation: bool,
    #[prost(string, tag = "3")]
    pub override_auto_sni_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlternateProtocolsCacheOptions {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub max_entries: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub key_value_store_config: ::core::option::Option<TypedExtensionConfig>,
    #[prost(message, repeated, tag = "4")]
    pub prepopulated_entries: ::prost::alloc::vec::Vec<
        alternate_protocols_cache_options::AlternateProtocolsCacheEntry,
    >,
    #[prost(string, repeated, tag = "5")]
    pub canonical_suffixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AlternateProtocolsCacheOptions`.
pub mod alternate_protocols_cache_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AlternateProtocolsCacheEntry {
        #[prost(string, tag = "1")]
        pub hostname: ::prost::alloc::string::String,
        #[prost(uint32, tag = "2")]
        pub port: u32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub max_connection_duration: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub max_headers_count: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "4")]
    pub max_stream_duration: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(
        enumeration = "http_protocol_options::HeadersWithUnderscoresAction",
        tag = "5"
    )]
    pub headers_with_underscores_action: i32,
    #[prost(message, optional, tag = "6")]
    pub max_requests_per_connection: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `HttpProtocolOptions`.
pub mod http_protocol_options {
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
    pub enum HeadersWithUnderscoresAction {
        Allow = 0,
        RejectRequest = 1,
        DropHeader = 2,
    }
    impl HeadersWithUnderscoresAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HeadersWithUnderscoresAction::Allow => "ALLOW",
                HeadersWithUnderscoresAction::RejectRequest => "REJECT_REQUEST",
                HeadersWithUnderscoresAction::DropHeader => "DROP_HEADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALLOW" => Some(Self::Allow),
                "REJECT_REQUEST" => Some(Self::RejectRequest),
                "DROP_HEADER" => Some(Self::DropHeader),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http1ProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub allow_absolute_url: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(bool, tag = "2")]
    pub accept_http_10: bool,
    #[prost(string, tag = "3")]
    pub default_host_for_http_10: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub header_key_format: ::core::option::Option<
        http1_protocol_options::HeaderKeyFormat,
    >,
    #[prost(bool, tag = "5")]
    pub enable_trailers: bool,
    #[prost(bool, tag = "6")]
    pub allow_chunked_length: bool,
    #[prost(message, optional, tag = "7")]
    pub override_stream_error_on_invalid_http_message: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(bool, tag = "8")]
    pub send_fully_qualified_url: bool,
    #[prost(message, optional, tag = "9")]
    pub use_balsa_parser: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(bool, tag = "10")]
    pub allow_custom_methods: bool,
}
/// Nested message and enum types in `Http1ProtocolOptions`.
pub mod http1_protocol_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderKeyFormat {
        #[prost(oneof = "header_key_format::HeaderFormat", tags = "1, 8")]
        pub header_format: ::core::option::Option<header_key_format::HeaderFormat>,
    }
    /// Nested message and enum types in `HeaderKeyFormat`.
    pub mod header_key_format {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct ProperCaseWords {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum HeaderFormat {
            #[prost(message, tag = "1")]
            ProperCaseWords(ProperCaseWords),
            #[prost(message, tag = "8")]
            StatefulFormatter(super::super::TypedExtensionConfig),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct KeepaliveSettings {
    #[prost(message, optional, tag = "1")]
    pub interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub interval_jitter: ::core::option::Option<
        super::super::super::r#type::v3::Percent,
    >,
    #[prost(message, optional, tag = "4")]
    pub connection_idle_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http2ProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub hpack_table_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "2")]
    pub max_concurrent_streams: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub initial_stream_window_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "4")]
    pub initial_connection_window_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(bool, tag = "5")]
    pub allow_connect: bool,
    #[prost(bool, tag = "6")]
    pub allow_metadata: bool,
    #[prost(message, optional, tag = "7")]
    pub max_outbound_frames: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "8")]
    pub max_outbound_control_frames: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "9")]
    pub max_consecutive_inbound_frames_with_empty_payload: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "10")]
    pub max_inbound_priority_frames_per_stream: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "11")]
    pub max_inbound_window_update_frames_per_data_frame_sent: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[deprecated]
    #[prost(bool, tag = "12")]
    pub stream_error_on_invalid_http_messaging: bool,
    #[prost(message, optional, tag = "14")]
    pub override_stream_error_on_invalid_http_message: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, repeated, tag = "13")]
    pub custom_settings_parameters: ::prost::alloc::vec::Vec<
        http2_protocol_options::SettingsParameter,
    >,
    #[prost(message, optional, tag = "15")]
    pub connection_keepalive: ::core::option::Option<KeepaliveSettings>,
    #[prost(message, optional, tag = "16")]
    pub use_oghttp2_codec: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
}
/// Nested message and enum types in `Http2ProtocolOptions`.
pub mod http2_protocol_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SettingsParameter {
        #[prost(message, optional, tag = "1")]
        pub identifier: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub http2_protocol_options: ::core::option::Option<Http2ProtocolOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http3ProtocolOptions {
    #[prost(message, optional, tag = "1")]
    pub quic_protocol_options: ::core::option::Option<QuicProtocolOptions>,
    #[prost(message, optional, tag = "2")]
    pub override_stream_error_on_invalid_http_message: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(bool, tag = "5")]
    pub allow_extended_connect: bool,
    #[prost(bool, tag = "6")]
    pub allow_metadata: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemeHeaderTransformation {
    #[prost(bool, tag = "2")]
    pub match_upstream: bool,
    #[prost(oneof = "scheme_header_transformation::Transformation", tags = "1")]
    pub transformation: ::core::option::Option<
        scheme_header_transformation::Transformation,
    >,
}
/// Nested message and enum types in `SchemeHeaderTransformation`.
pub mod scheme_header_transformation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transformation {
        #[prost(string, tag = "1")]
        SchemeToOverwrite(::prost::alloc::string::String),
    }
}
