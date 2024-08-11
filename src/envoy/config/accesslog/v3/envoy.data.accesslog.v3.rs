// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpAccessLogEntry {
    #[prost(message, optional, tag = "1")]
    pub common_properties: ::core::option::Option<AccessLogCommon>,
    #[prost(message, optional, tag = "2")]
    pub connection_properties: ::core::option::Option<ConnectionProperties>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpAccessLogEntry {
    #[prost(message, optional, tag = "1")]
    pub common_properties: ::core::option::Option<AccessLogCommon>,
    #[prost(enumeration = "http_access_log_entry::HttpVersion", tag = "2")]
    pub protocol_version: i32,
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<HttpRequestProperties>,
    #[prost(message, optional, tag = "4")]
    pub response: ::core::option::Option<HttpResponseProperties>,
}
/// Nested message and enum types in `HTTPAccessLogEntry`.
pub mod http_access_log_entry {
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
    pub enum HttpVersion {
        ProtocolUnspecified = 0,
        Http10 = 1,
        Http11 = 2,
        Http2 = 3,
        Http3 = 4,
    }
    impl HttpVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HttpVersion::ProtocolUnspecified => "PROTOCOL_UNSPECIFIED",
                HttpVersion::Http10 => "HTTP10",
                HttpVersion::Http11 => "HTTP11",
                HttpVersion::Http2 => "HTTP2",
                HttpVersion::Http3 => "HTTP3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::ProtocolUnspecified),
                "HTTP10" => Some(Self::Http10),
                "HTTP11" => Some(Self::Http11),
                "HTTP2" => Some(Self::Http2),
                "HTTP3" => Some(Self::Http3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConnectionProperties {
    #[prost(uint64, tag = "1")]
    pub received_bytes: u64,
    #[prost(uint64, tag = "2")]
    pub sent_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLogCommon {
    #[prost(double, tag = "1")]
    pub sample_rate: f64,
    #[prost(message, optional, tag = "2")]
    pub downstream_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(message, optional, tag = "3")]
    pub downstream_local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(message, optional, tag = "4")]
    pub tls_properties: ::core::option::Option<TlsProperties>,
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(message, optional, tag = "6")]
    pub time_to_last_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "7")]
    pub time_to_first_upstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "8")]
    pub time_to_last_upstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "9")]
    pub time_to_first_upstream_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "10")]
    pub time_to_last_upstream_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "11")]
    pub time_to_first_downstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "12")]
    pub time_to_last_downstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "13")]
    pub upstream_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(message, optional, tag = "14")]
    pub upstream_local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(string, tag = "15")]
    pub upstream_cluster: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "16")]
    pub response_flags: ::core::option::Option<ResponseFlags>,
    #[prost(message, optional, tag = "17")]
    pub metadata: ::core::option::Option<
        super::super::super::config::core::v3::Metadata,
    >,
    #[prost(string, tag = "18")]
    pub upstream_transport_failure_reason: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub route_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "20")]
    pub downstream_direct_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(map = "string, message", tag = "21")]
    pub filter_state_objects: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    #[prost(map = "string, string", tag = "22")]
    pub custom_tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "23")]
    pub duration: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(uint32, tag = "24")]
    pub upstream_request_attempt_count: u32,
    #[prost(string, tag = "25")]
    pub connection_termination_details: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub stream_id: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(bool, tag = "27")]
    pub intermediate_log_entry: bool,
    #[prost(string, tag = "28")]
    pub downstream_transport_failure_reason: ::prost::alloc::string::String,
    #[prost(uint64, tag = "29")]
    pub downstream_wire_bytes_sent: u64,
    #[prost(uint64, tag = "30")]
    pub downstream_wire_bytes_received: u64,
    #[prost(uint64, tag = "31")]
    pub upstream_wire_bytes_sent: u64,
    #[prost(uint64, tag = "32")]
    pub upstream_wire_bytes_received: u64,
    #[prost(enumeration = "AccessLogType", tag = "33")]
    pub access_log_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ResponseFlags {
    #[prost(bool, tag = "1")]
    pub failed_local_healthcheck: bool,
    #[prost(bool, tag = "2")]
    pub no_healthy_upstream: bool,
    #[prost(bool, tag = "3")]
    pub upstream_request_timeout: bool,
    #[prost(bool, tag = "4")]
    pub local_reset: bool,
    #[prost(bool, tag = "5")]
    pub upstream_remote_reset: bool,
    #[prost(bool, tag = "6")]
    pub upstream_connection_failure: bool,
    #[prost(bool, tag = "7")]
    pub upstream_connection_termination: bool,
    #[prost(bool, tag = "8")]
    pub upstream_overflow: bool,
    #[prost(bool, tag = "9")]
    pub no_route_found: bool,
    #[prost(bool, tag = "10")]
    pub delay_injected: bool,
    #[prost(bool, tag = "11")]
    pub fault_injected: bool,
    #[prost(bool, tag = "12")]
    pub rate_limited: bool,
    #[prost(message, optional, tag = "13")]
    pub unauthorized_details: ::core::option::Option<response_flags::Unauthorized>,
    #[prost(bool, tag = "14")]
    pub rate_limit_service_error: bool,
    #[prost(bool, tag = "15")]
    pub downstream_connection_termination: bool,
    #[prost(bool, tag = "16")]
    pub upstream_retry_limit_exceeded: bool,
    #[prost(bool, tag = "17")]
    pub stream_idle_timeout: bool,
    #[prost(bool, tag = "18")]
    pub invalid_envoy_request_headers: bool,
    #[prost(bool, tag = "19")]
    pub downstream_protocol_error: bool,
    #[prost(bool, tag = "20")]
    pub upstream_max_stream_duration_reached: bool,
    #[prost(bool, tag = "21")]
    pub response_from_cache_filter: bool,
    #[prost(bool, tag = "22")]
    pub no_filter_config_found: bool,
    #[prost(bool, tag = "23")]
    pub duration_timeout: bool,
    #[prost(bool, tag = "24")]
    pub upstream_protocol_error: bool,
    #[prost(bool, tag = "25")]
    pub no_cluster_found: bool,
    #[prost(bool, tag = "26")]
    pub overload_manager: bool,
    #[prost(bool, tag = "27")]
    pub dns_resolution_failure: bool,
    #[prost(bool, tag = "28")]
    pub downstream_remote_reset: bool,
}
/// Nested message and enum types in `ResponseFlags`.
pub mod response_flags {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Unauthorized {
        #[prost(enumeration = "unauthorized::Reason", tag = "1")]
        pub reason: i32,
    }
    /// Nested message and enum types in `Unauthorized`.
    pub mod unauthorized {
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
        pub enum Reason {
            Unspecified = 0,
            ExternalService = 1,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::ExternalService => "EXTERNAL_SERVICE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "EXTERNAL_SERVICE" => Some(Self::ExternalService),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsProperties {
    #[prost(enumeration = "tls_properties::TlsVersion", tag = "1")]
    pub tls_version: i32,
    #[prost(message, optional, tag = "2")]
    pub tls_cipher_suite: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(string, tag = "3")]
    pub tls_sni_hostname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub local_certificate_properties: ::core::option::Option<
        tls_properties::CertificateProperties,
    >,
    #[prost(message, optional, tag = "5")]
    pub peer_certificate_properties: ::core::option::Option<
        tls_properties::CertificateProperties,
    >,
    #[prost(string, tag = "6")]
    pub tls_session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub ja3_fingerprint: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TLSProperties`.
pub mod tls_properties {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateProperties {
        #[prost(message, repeated, tag = "1")]
        pub subject_alt_name: ::prost::alloc::vec::Vec<
            certificate_properties::SubjectAltName,
        >,
        #[prost(string, tag = "2")]
        pub subject: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub issuer: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `CertificateProperties`.
    pub mod certificate_properties {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SubjectAltName {
            #[prost(oneof = "subject_alt_name::San", tags = "1, 2")]
            pub san: ::core::option::Option<subject_alt_name::San>,
        }
        /// Nested message and enum types in `SubjectAltName`.
        pub mod subject_alt_name {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum San {
                #[prost(string, tag = "1")]
                Uri(::prost::alloc::string::String),
                #[prost(string, tag = "2")]
                Dns(::prost::alloc::string::String),
            }
        }
    }
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
    pub enum TlsVersion {
        VersionUnspecified = 0,
        TlSv1 = 1,
        TlSv11 = 2,
        TlSv12 = 3,
        TlSv13 = 4,
    }
    impl TlsVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsVersion::VersionUnspecified => "VERSION_UNSPECIFIED",
                TlsVersion::TlSv1 => "TLSv1",
                TlsVersion::TlSv11 => "TLSv1_1",
                TlsVersion::TlSv12 => "TLSv1_2",
                TlsVersion::TlSv13 => "TLSv1_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_UNSPECIFIED" => Some(Self::VersionUnspecified),
                "TLSv1" => Some(Self::TlSv1),
                "TLSv1_1" => Some(Self::TlSv11),
                "TLSv1_2" => Some(Self::TlSv12),
                "TLSv1_3" => Some(Self::TlSv13),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestProperties {
    #[prost(
        enumeration = "super::super::super::config::core::v3::RequestMethod",
        tag = "1"
    )]
    pub request_method: i32,
    #[prost(string, tag = "2")]
    pub scheme: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub port: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub referer: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub forwarded_for: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub original_path: ::prost::alloc::string::String,
    #[prost(uint64, tag = "11")]
    pub request_headers_bytes: u64,
    #[prost(uint64, tag = "12")]
    pub request_body_bytes: u64,
    #[prost(map = "string, string", tag = "13")]
    pub request_headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(uint64, tag = "14")]
    pub upstream_header_bytes_sent: u64,
    #[prost(uint64, tag = "15")]
    pub downstream_header_bytes_received: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponseProperties {
    #[prost(message, optional, tag = "1")]
    pub response_code: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(uint64, tag = "2")]
    pub response_headers_bytes: u64,
    #[prost(uint64, tag = "3")]
    pub response_body_bytes: u64,
    #[prost(map = "string, string", tag = "4")]
    pub response_headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "5")]
    pub response_trailers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "6")]
    pub response_code_details: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub upstream_header_bytes_received: u64,
    #[prost(uint64, tag = "8")]
    pub downstream_header_bytes_sent: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLogType {
    NotSet = 0,
    TcpUpstreamConnected = 1,
    TcpPeriodic = 2,
    TcpConnectionEnd = 3,
    DownstreamStart = 4,
    DownstreamPeriodic = 5,
    DownstreamEnd = 6,
    UpstreamPoolReady = 7,
    UpstreamPeriodic = 8,
    UpstreamEnd = 9,
    DownstreamTunnelSuccessfullyEstablished = 10,
    UdpTunnelUpstreamConnected = 11,
    UdpPeriodic = 12,
    UdpSessionEnd = 13,
}
impl AccessLogType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessLogType::NotSet => "NotSet",
            AccessLogType::TcpUpstreamConnected => "TcpUpstreamConnected",
            AccessLogType::TcpPeriodic => "TcpPeriodic",
            AccessLogType::TcpConnectionEnd => "TcpConnectionEnd",
            AccessLogType::DownstreamStart => "DownstreamStart",
            AccessLogType::DownstreamPeriodic => "DownstreamPeriodic",
            AccessLogType::DownstreamEnd => "DownstreamEnd",
            AccessLogType::UpstreamPoolReady => "UpstreamPoolReady",
            AccessLogType::UpstreamPeriodic => "UpstreamPeriodic",
            AccessLogType::UpstreamEnd => "UpstreamEnd",
            AccessLogType::DownstreamTunnelSuccessfullyEstablished => {
                "DownstreamTunnelSuccessfullyEstablished"
            }
            AccessLogType::UdpTunnelUpstreamConnected => "UdpTunnelUpstreamConnected",
            AccessLogType::UdpPeriodic => "UdpPeriodic",
            AccessLogType::UdpSessionEnd => "UdpSessionEnd",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotSet" => Some(Self::NotSet),
            "TcpUpstreamConnected" => Some(Self::TcpUpstreamConnected),
            "TcpPeriodic" => Some(Self::TcpPeriodic),
            "TcpConnectionEnd" => Some(Self::TcpConnectionEnd),
            "DownstreamStart" => Some(Self::DownstreamStart),
            "DownstreamPeriodic" => Some(Self::DownstreamPeriodic),
            "DownstreamEnd" => Some(Self::DownstreamEnd),
            "UpstreamPoolReady" => Some(Self::UpstreamPoolReady),
            "UpstreamPeriodic" => Some(Self::UpstreamPeriodic),
            "UpstreamEnd" => Some(Self::UpstreamEnd),
            "DownstreamTunnelSuccessfullyEstablished" => {
                Some(Self::DownstreamTunnelSuccessfullyEstablished)
            }
            "UdpTunnelUpstreamConnected" => Some(Self::UdpTunnelUpstreamConnected),
            "UdpPeriodic" => Some(Self::UdpPeriodic),
            "UdpSessionEnd" => Some(Self::UdpSessionEnd),
            _ => None,
        }
    }
}
