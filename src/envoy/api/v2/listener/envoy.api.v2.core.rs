// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketOption {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub level: i64,
    #[prost(int64, tag = "3")]
    pub name: i64,
    #[prost(enumeration = "socket_option::SocketState", tag = "6")]
    pub state: i32,
    #[prost(oneof = "socket_option::Value", tags = "4, 5")]
    pub value: ::core::option::Option<socket_option::Value>,
}
/// Nested message and enum types in `SocketOption`.
pub mod socket_option {
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
    pub enum SocketState {
        StatePrebind = 0,
        StateBound = 1,
        StateListening = 2,
    }
    impl SocketState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SocketState::StatePrebind => "STATE_PREBIND",
                SocketState::StateBound => "STATE_BOUND",
                SocketState::StateListening => "STATE_LISTENING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_PREBIND" => Some(Self::StatePrebind),
                "STATE_BOUND" => Some(Self::StateBound),
                "STATE_LISTENING" => Some(Self::StateListening),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int64, tag = "4")]
        IntValue(i64),
        #[prost(bytes, tag = "5")]
        BufValue(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipe {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub mode: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketAddress {
    #[prost(enumeration = "socket_address::Protocol", tag = "1")]
    pub protocol: i32,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub resolver_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub ipv4_compat: bool,
    #[prost(oneof = "socket_address::PortSpecifier", tags = "3, 4")]
    pub port_specifier: ::core::option::Option<socket_address::PortSpecifier>,
}
/// Nested message and enum types in `SocketAddress`.
pub mod socket_address {
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
    pub enum Protocol {
        Tcp = 0,
        Udp = 1,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Protocol::Tcp => "TCP",
                Protocol::Udp => "UDP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TCP" => Some(Self::Tcp),
                "UDP" => Some(Self::Udp),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PortSpecifier {
        #[prost(uint32, tag = "3")]
        PortValue(u32),
        #[prost(string, tag = "4")]
        NamedPort(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TcpKeepalive {
    #[prost(message, optional, tag = "1")]
    pub keepalive_probes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "2")]
    pub keepalive_time: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub keepalive_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindConfig {
    #[prost(message, optional, tag = "1")]
    pub source_address: ::core::option::Option<SocketAddress>,
    #[prost(message, optional, tag = "2")]
    pub freebind: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, repeated, tag = "3")]
    pub socket_options: ::prost::alloc::vec::Vec<SocketOption>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(oneof = "address::Address", tags = "1, 2")]
    pub address: ::core::option::Option<address::Address>,
}
/// Nested message and enum types in `Address`.
pub mod address {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Address {
        #[prost(message, tag = "1")]
        SocketAddress(super::SocketAddress),
        #[prost(message, tag = "2")]
        Pipe(super::Pipe),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CidrRange {
    #[prost(string, tag = "1")]
    pub address_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub prefix_len: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BackoffStrategy {
    #[prost(message, optional, tag = "1")]
    pub base_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub max_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpUri {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(oneof = "http_uri::HttpUpstreamType", tags = "2")]
    pub http_upstream_type: ::core::option::Option<http_uri::HttpUpstreamType>,
}
/// Nested message and enum types in `HttpUri`.
pub mod http_uri {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HttpUpstreamType {
        #[prost(string, tag = "2")]
        Cluster(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locality {
    #[prost(string, tag = "1")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sub_zone: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildVersion {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::super::super::r#type::SemanticVersion>,
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub type_descriptor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<BuildVersion>,
    #[prost(bool, tag = "5")]
    pub disabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
    #[prost(message, optional, tag = "4")]
    pub locality: ::core::option::Option<Locality>,
    #[deprecated]
    #[prost(string, tag = "5")]
    pub build_version: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub user_agent_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(string, repeated, tag = "10")]
    pub client_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "11")]
    pub listening_addresses: ::prost::alloc::vec::Vec<Address>,
    #[prost(oneof = "node::UserAgentVersionType", tags = "7, 8")]
    pub user_agent_version_type: ::core::option::Option<node::UserAgentVersionType>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserAgentVersionType {
        #[prost(string, tag = "7")]
        UserAgentVersion(::prost::alloc::string::String),
        #[prost(message, tag = "8")]
        UserAgentBuildVersion(super::BuildVersion),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(map = "string, message", tag = "1")]
    pub filter_metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeUInt32 {
    #[prost(uint32, tag = "2")]
    pub default_value: u32,
    #[prost(string, tag = "3")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeDouble {
    #[prost(double, tag = "1")]
    pub default_value: f64,
    #[prost(string, tag = "2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFeatureFlag {
    #[prost(message, optional, tag = "1")]
    pub default_value: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(string, tag = "2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValueOption {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<HeaderValue>,
    #[prost(message, optional, tag = "2")]
    pub append: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMap {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<HeaderValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    #[prost(oneof = "data_source::Specifier", tags = "1, 2, 3")]
    pub specifier: ::core::option::Option<data_source::Specifier>,
}
/// Nested message and enum types in `DataSource`.
pub mod data_source {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Specifier {
        #[prost(string, tag = "1")]
        Filename(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        InlineBytes(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "3")]
        InlineString(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    #[prost(message, optional, tag = "1")]
    pub retry_back_off: ::core::option::Option<BackoffStrategy>,
    #[prost(message, optional, tag = "2")]
    pub num_retries: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteDataSource {
    #[prost(message, optional, tag = "1")]
    pub http_uri: ::core::option::Option<HttpUri>,
    #[prost(string, tag = "2")]
    pub sha256: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncDataSource {
    #[prost(oneof = "async_data_source::Specifier", tags = "1, 2")]
    pub specifier: ::core::option::Option<async_data_source::Specifier>,
}
/// Nested message and enum types in `AsyncDataSource`.
pub mod async_data_source {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Specifier {
        #[prost(message, tag = "1")]
        Local(super::DataSource),
        #[prost(message, tag = "2")]
        Remote(super::RemoteDataSource),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransportSocket {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "transport_socket::ConfigType", tags = "2, 3")]
    pub config_type: ::core::option::Option<transport_socket::ConfigType>,
}
/// Nested message and enum types in `TransportSocket`.
pub mod transport_socket {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "2")]
        Config(super::super::super::super::super::google::protobuf::Struct),
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFractionalPercent {
    #[prost(message, optional, tag = "1")]
    pub default_value: ::core::option::Option<
        super::super::super::r#type::FractionalPercent,
    >,
    #[prost(string, tag = "2")]
    pub runtime_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlPlane {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoutingPriority {
    Default = 0,
    High = 1,
}
impl RoutingPriority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoutingPriority::Default => "DEFAULT",
            RoutingPriority::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestMethod {
    MethodUnspecified = 0,
    Get = 1,
    Head = 2,
    Post = 3,
    Put = 4,
    Delete = 5,
    Connect = 6,
    Options = 7,
    Trace = 8,
    Patch = 9,
}
impl RequestMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RequestMethod::MethodUnspecified => "METHOD_UNSPECIFIED",
            RequestMethod::Get => "GET",
            RequestMethod::Head => "HEAD",
            RequestMethod::Post => "POST",
            RequestMethod::Put => "PUT",
            RequestMethod::Delete => "DELETE",
            RequestMethod::Connect => "CONNECT",
            RequestMethod::Options => "OPTIONS",
            RequestMethod::Trace => "TRACE",
            RequestMethod::Patch => "PATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "METHOD_UNSPECIFIED" => Some(Self::MethodUnspecified),
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "POST" => Some(Self::Post),
            "PUT" => Some(Self::Put),
            "DELETE" => Some(Self::Delete),
            "CONNECT" => Some(Self::Connect),
            "OPTIONS" => Some(Self::Options),
            "TRACE" => Some(Self::Trace),
            "PATCH" => Some(Self::Patch),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}
impl TrafficDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrafficDirection::Unspecified => "UNSPECIFIED",
            TrafficDirection::Inbound => "INBOUND",
            TrafficDirection::Outbound => "OUTBOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "INBOUND" => Some(Self::Inbound),
            "OUTBOUND" => Some(Self::Outbound),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcService {
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, repeated, tag = "5")]
    pub initial_metadata: ::prost::alloc::vec::Vec<HeaderValue>,
    #[prost(oneof = "grpc_service::TargetSpecifier", tags = "1, 2")]
    pub target_specifier: ::core::option::Option<grpc_service::TargetSpecifier>,
}
/// Nested message and enum types in `GrpcService`.
pub mod grpc_service {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnvoyGrpc {
        #[prost(string, tag = "1")]
        pub cluster_name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoogleGrpc {
        #[prost(string, tag = "1")]
        pub target_uri: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub channel_credentials: ::core::option::Option<google_grpc::ChannelCredentials>,
        #[prost(message, repeated, tag = "3")]
        pub call_credentials: ::prost::alloc::vec::Vec<google_grpc::CallCredentials>,
        #[prost(string, tag = "4")]
        pub stat_prefix: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub credentials_factory_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "6")]
        pub config: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Struct,
        >,
    }
    /// Nested message and enum types in `GoogleGrpc`.
    pub mod google_grpc {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SslCredentials {
            #[prost(message, optional, tag = "1")]
            pub root_certs: ::core::option::Option<super::super::DataSource>,
            #[prost(message, optional, tag = "2")]
            pub private_key: ::core::option::Option<super::super::DataSource>,
            #[prost(message, optional, tag = "3")]
            pub cert_chain: ::core::option::Option<super::super::DataSource>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct GoogleLocalCredentials {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ChannelCredentials {
            #[prost(
                oneof = "channel_credentials::CredentialSpecifier",
                tags = "1, 2, 3"
            )]
            pub credential_specifier: ::core::option::Option<
                channel_credentials::CredentialSpecifier,
            >,
        }
        /// Nested message and enum types in `ChannelCredentials`.
        pub mod channel_credentials {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum CredentialSpecifier {
                #[prost(message, tag = "1")]
                SslCredentials(super::SslCredentials),
                #[prost(message, tag = "2")]
                GoogleDefault(
                    super::super::super::super::super::super::super::google::protobuf::Empty,
                ),
                #[prost(message, tag = "3")]
                LocalCredentials(super::GoogleLocalCredentials),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CallCredentials {
            #[prost(
                oneof = "call_credentials::CredentialSpecifier",
                tags = "1, 2, 3, 4, 5, 6, 7"
            )]
            pub credential_specifier: ::core::option::Option<
                call_credentials::CredentialSpecifier,
            >,
        }
        /// Nested message and enum types in `CallCredentials`.
        pub mod call_credentials {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ServiceAccountJwtAccessCredentials {
                #[prost(string, tag = "1")]
                pub json_key: ::prost::alloc::string::String,
                #[prost(uint64, tag = "2")]
                pub token_lifetime_seconds: u64,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GoogleIamCredentials {
                #[prost(string, tag = "1")]
                pub authorization_token: ::prost::alloc::string::String,
                #[prost(string, tag = "2")]
                pub authority_selector: ::prost::alloc::string::String,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MetadataCredentialsFromPlugin {
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                #[prost(
                    oneof = "metadata_credentials_from_plugin::ConfigType",
                    tags = "2, 3"
                )]
                pub config_type: ::core::option::Option<
                    metadata_credentials_from_plugin::ConfigType,
                >,
            }
            /// Nested message and enum types in `MetadataCredentialsFromPlugin`.
            pub mod metadata_credentials_from_plugin {
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum ConfigType {
                    #[prost(message, tag = "2")]
                    Config(
                        super::super::super::super::super::super::super::super::google::protobuf::Struct,
                    ),
                    #[prost(message, tag = "3")]
                    TypedConfig(
                        super::super::super::super::super::super::super::super::google::protobuf::Any,
                    ),
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct StsService {
                #[prost(string, tag = "1")]
                pub token_exchange_service_uri: ::prost::alloc::string::String,
                #[prost(string, tag = "2")]
                pub resource: ::prost::alloc::string::String,
                #[prost(string, tag = "3")]
                pub audience: ::prost::alloc::string::String,
                #[prost(string, tag = "4")]
                pub scope: ::prost::alloc::string::String,
                #[prost(string, tag = "5")]
                pub requested_token_type: ::prost::alloc::string::String,
                #[prost(string, tag = "6")]
                pub subject_token_path: ::prost::alloc::string::String,
                #[prost(string, tag = "7")]
                pub subject_token_type: ::prost::alloc::string::String,
                #[prost(string, tag = "8")]
                pub actor_token_path: ::prost::alloc::string::String,
                #[prost(string, tag = "9")]
                pub actor_token_type: ::prost::alloc::string::String,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum CredentialSpecifier {
                #[prost(string, tag = "1")]
                AccessToken(::prost::alloc::string::String),
                #[prost(message, tag = "2")]
                GoogleComputeEngine(
                    super::super::super::super::super::super::super::google::protobuf::Empty,
                ),
                #[prost(string, tag = "3")]
                GoogleRefreshToken(::prost::alloc::string::String),
                #[prost(message, tag = "4")]
                ServiceAccountJwtAccess(ServiceAccountJwtAccessCredentials),
                #[prost(message, tag = "5")]
                GoogleIam(GoogleIamCredentials),
                #[prost(message, tag = "6")]
                FromPlugin(MetadataCredentialsFromPlugin),
                #[prost(message, tag = "7")]
                StsService(StsService),
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetSpecifier {
        #[prost(message, tag = "1")]
        EnvoyGrpc(EnvoyGrpc),
        #[prost(message, tag = "2")]
        GoogleGrpc(GoogleGrpc),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfigSource {
    #[prost(enumeration = "api_config_source::ApiType", tag = "1")]
    pub api_type: i32,
    #[prost(enumeration = "ApiVersion", tag = "8")]
    pub transport_api_version: i32,
    #[prost(string, repeated, tag = "2")]
    pub cluster_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub grpc_services: ::prost::alloc::vec::Vec<GrpcService>,
    #[prost(message, optional, tag = "3")]
    pub refresh_delay: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "5")]
    pub request_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "6")]
    pub rate_limit_settings: ::core::option::Option<RateLimitSettings>,
    #[prost(bool, tag = "7")]
    pub set_node_on_first_message_only: bool,
}
/// Nested message and enum types in `ApiConfigSource`.
pub mod api_config_source {
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
    pub enum ApiType {
        UnsupportedRestLegacy = 0,
        Rest = 1,
        Grpc = 2,
        DeltaGrpc = 3,
    }
    impl ApiType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiType::UnsupportedRestLegacy => "UNSUPPORTED_REST_LEGACY",
                ApiType::Rest => "REST",
                ApiType::Grpc => "GRPC",
                ApiType::DeltaGrpc => "DELTA_GRPC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSUPPORTED_REST_LEGACY" => Some(Self::UnsupportedRestLegacy),
                "REST" => Some(Self::Rest),
                "GRPC" => Some(Self::Grpc),
                "DELTA_GRPC" => Some(Self::DeltaGrpc),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AggregatedConfigSource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SelfConfigSource {
    #[prost(enumeration = "ApiVersion", tag = "1")]
    pub transport_api_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RateLimitSettings {
    #[prost(message, optional, tag = "1")]
    pub max_tokens: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "2")]
    pub fill_rate: ::core::option::Option<
        super::super::super::super::google::protobuf::DoubleValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSource {
    #[prost(message, optional, tag = "4")]
    pub initial_fetch_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(enumeration = "ApiVersion", tag = "6")]
    pub resource_api_version: i32,
    #[prost(oneof = "config_source::ConfigSourceSpecifier", tags = "1, 2, 3, 5")]
    pub config_source_specifier: ::core::option::Option<
        config_source::ConfigSourceSpecifier,
    >,
}
/// Nested message and enum types in `ConfigSource`.
pub mod config_source {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigSourceSpecifier {
        #[prost(string, tag = "1")]
        Path(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        ApiConfigSource(super::ApiConfigSource),
        #[prost(message, tag = "3")]
        Ads(super::AggregatedConfigSource),
        #[prost(message, tag = "5")]
        Self_(super::SelfConfigSource),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiVersion {
    Auto = 0,
    V2 = 1,
    V3 = 2,
}
impl ApiVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiVersion::Auto => "AUTO",
            ApiVersion::V2 => "V2",
            ApiVersion::V3 => "V3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO" => Some(Self::Auto),
            "V2" => Some(Self::V2),
            "V3" => Some(Self::V3),
            _ => None,
        }
    }
}
