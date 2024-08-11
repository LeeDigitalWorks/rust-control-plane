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
    #[prost(message, optional, tag = "7")]
    pub r#type: ::core::option::Option<socket_option::SocketType>,
    #[prost(oneof = "socket_option::Value", tags = "4, 5")]
    pub value: ::core::option::Option<socket_option::Value>,
}
/// Nested message and enum types in `SocketOption`.
pub mod socket_option {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SocketType {
        #[prost(message, optional, tag = "1")]
        pub stream: ::core::option::Option<socket_type::Stream>,
        #[prost(message, optional, tag = "2")]
        pub datagram: ::core::option::Option<socket_type::Datagram>,
    }
    /// Nested message and enum types in `SocketType`.
    pub mod socket_type {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Stream {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Datagram {}
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
pub struct SocketOptionsOverride {
    #[prost(message, repeated, tag = "1")]
    pub socket_options: ::prost::alloc::vec::Vec<SocketOption>,
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
pub struct EnvoyInternalAddress {
    #[prost(string, tag = "2")]
    pub endpoint_id: ::prost::alloc::string::String,
    #[prost(oneof = "envoy_internal_address::AddressNameSpecifier", tags = "1")]
    pub address_name_specifier: ::core::option::Option<
        envoy_internal_address::AddressNameSpecifier,
    >,
}
/// Nested message and enum types in `EnvoyInternalAddress`.
pub mod envoy_internal_address {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AddressNameSpecifier {
        #[prost(string, tag = "1")]
        ServerListenerName(::prost::alloc::string::String),
    }
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
pub struct ExtraSourceAddress {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<SocketAddress>,
    #[prost(message, optional, tag = "2")]
    pub socket_options: ::core::option::Option<SocketOptionsOverride>,
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
    #[prost(message, repeated, tag = "5")]
    pub extra_source_addresses: ::prost::alloc::vec::Vec<ExtraSourceAddress>,
    #[deprecated]
    #[prost(message, repeated, tag = "4")]
    pub additional_source_addresses: ::prost::alloc::vec::Vec<SocketAddress>,
    #[prost(message, optional, tag = "6")]
    pub local_address_selector: ::core::option::Option<TypedExtensionConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(oneof = "address::Address", tags = "1, 2, 3")]
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
        #[prost(message, tag = "3")]
        EnvoyInternalAddress(super::EnvoyInternalAddress),
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
    pub version: ::core::option::Option<
        super::super::super::r#type::v3::SemanticVersion,
    >,
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
    #[deprecated]
    #[prost(string, tag = "3")]
    pub type_descriptor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<BuildVersion>,
    #[prost(bool, tag = "5")]
    pub disabled: bool,
    #[prost(string, repeated, tag = "6")]
    pub type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    #[prost(map = "string, message", tag = "12")]
    pub dynamic_parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::xds::core::v3::ContextParams,
    >,
    #[prost(message, optional, tag = "4")]
    pub locality: ::core::option::Option<Locality>,
    #[prost(string, tag = "6")]
    pub user_agent_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(string, repeated, tag = "10")]
    pub client_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
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
    #[prost(map = "string, message", tag = "2")]
    pub typed_filter_metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
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
pub struct RuntimePercent {
    #[prost(message, optional, tag = "1")]
    pub default_value: ::core::option::Option<super::super::super::r#type::v3::Percent>,
    #[prost(string, tag = "2")]
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
pub struct KeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueAppend {
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<KeyValue>,
    #[prost(enumeration = "key_value_append::KeyValueAppendAction", tag = "2")]
    pub action: i32,
}
/// Nested message and enum types in `KeyValueAppend`.
pub mod key_value_append {
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
    pub enum KeyValueAppendAction {
        AppendIfExistsOrAdd = 0,
        AddIfAbsent = 1,
        OverwriteIfExistsOrAdd = 2,
        OverwriteIfExists = 3,
    }
    impl KeyValueAppendAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeyValueAppendAction::AppendIfExistsOrAdd => "APPEND_IF_EXISTS_OR_ADD",
                KeyValueAppendAction::AddIfAbsent => "ADD_IF_ABSENT",
                KeyValueAppendAction::OverwriteIfExistsOrAdd => {
                    "OVERWRITE_IF_EXISTS_OR_ADD"
                }
                KeyValueAppendAction::OverwriteIfExists => "OVERWRITE_IF_EXISTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APPEND_IF_EXISTS_OR_ADD" => Some(Self::AppendIfExistsOrAdd),
                "ADD_IF_ABSENT" => Some(Self::AddIfAbsent),
                "OVERWRITE_IF_EXISTS_OR_ADD" => Some(Self::OverwriteIfExistsOrAdd),
                "OVERWRITE_IF_EXISTS" => Some(Self::OverwriteIfExists),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueMutation {
    #[prost(message, optional, tag = "1")]
    pub append: ::core::option::Option<KeyValueAppend>,
    #[prost(string, tag = "2")]
    pub remove: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameter {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub raw_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValueOption {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<HeaderValue>,
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub append: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(enumeration = "header_value_option::HeaderAppendAction", tag = "3")]
    pub append_action: i32,
    #[prost(bool, tag = "4")]
    pub keep_empty_value: bool,
}
/// Nested message and enum types in `HeaderValueOption`.
pub mod header_value_option {
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
    pub enum HeaderAppendAction {
        AppendIfExistsOrAdd = 0,
        AddIfAbsent = 1,
        OverwriteIfExistsOrAdd = 2,
        OverwriteIfExists = 3,
    }
    impl HeaderAppendAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HeaderAppendAction::AppendIfExistsOrAdd => "APPEND_IF_EXISTS_OR_ADD",
                HeaderAppendAction::AddIfAbsent => "ADD_IF_ABSENT",
                HeaderAppendAction::OverwriteIfExistsOrAdd => {
                    "OVERWRITE_IF_EXISTS_OR_ADD"
                }
                HeaderAppendAction::OverwriteIfExists => "OVERWRITE_IF_EXISTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APPEND_IF_EXISTS_OR_ADD" => Some(Self::AppendIfExistsOrAdd),
                "ADD_IF_ABSENT" => Some(Self::AddIfAbsent),
                "OVERWRITE_IF_EXISTS_OR_ADD" => Some(Self::OverwriteIfExistsOrAdd),
                "OVERWRITE_IF_EXISTS" => Some(Self::OverwriteIfExists),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMap {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<HeaderValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchedDirectory {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    #[prost(message, optional, tag = "5")]
    pub watched_directory: ::core::option::Option<WatchedDirectory>,
    #[prost(oneof = "data_source::Specifier", tags = "1, 2, 3, 4")]
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
        #[prost(string, tag = "4")]
        EnvironmentVariable(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    #[prost(message, optional, tag = "1")]
    pub retry_back_off: ::core::option::Option<BackoffStrategy>,
    #[prost(message, optional, tag = "2")]
    pub num_retries: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(string, tag = "3")]
    pub retry_on: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub retry_priority: ::core::option::Option<retry_policy::RetryPriority>,
    #[prost(message, repeated, tag = "5")]
    pub retry_host_predicate: ::prost::alloc::vec::Vec<retry_policy::RetryHostPredicate>,
    #[prost(int64, tag = "6")]
    pub host_selection_retry_max_attempts: i64,
}
/// Nested message and enum types in `RetryPolicy`.
pub mod retry_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPriority {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "retry_priority::ConfigType", tags = "2")]
        pub config_type: ::core::option::Option<retry_priority::ConfigType>,
    }
    /// Nested message and enum types in `RetryPriority`.
    pub mod retry_priority {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "2")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryHostPredicate {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "retry_host_predicate::ConfigType", tags = "2")]
        pub config_type: ::core::option::Option<retry_host_predicate::ConfigType>,
    }
    /// Nested message and enum types in `RetryHostPredicate`.
    pub mod retry_host_predicate {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "2")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
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
    #[prost(oneof = "transport_socket::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<transport_socket::ConfigType>,
}
/// Nested message and enum types in `TransportSocket`.
pub mod transport_socket {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFractionalPercent {
    #[prost(message, optional, tag = "1")]
    pub default_value: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
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
pub struct ProxyProtocolPassThroughTlVs {
    #[prost(
        enumeration = "proxy_protocol_pass_through_tl_vs::PassTlVsMatchType",
        tag = "1"
    )]
    pub match_type: i32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub tlv_type: ::prost::alloc::vec::Vec<u32>,
}
/// Nested message and enum types in `ProxyProtocolPassThroughTLVs`.
pub mod proxy_protocol_pass_through_tl_vs {
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
    pub enum PassTlVsMatchType {
        IncludeAll = 0,
        Include = 1,
    }
    impl PassTlVsMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PassTlVsMatchType::IncludeAll => "INCLUDE_ALL",
                PassTlVsMatchType::Include => "INCLUDE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INCLUDE_ALL" => Some(Self::IncludeAll),
                "INCLUDE" => Some(Self::Include),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocolConfig {
    #[prost(enumeration = "proxy_protocol_config::Version", tag = "1")]
    pub version: i32,
    #[prost(message, optional, tag = "2")]
    pub pass_through_tlvs: ::core::option::Option<ProxyProtocolPassThroughTlVs>,
}
/// Nested message and enum types in `ProxyProtocolConfig`.
pub mod proxy_protocol_config {
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
    pub enum Version {
        V1 = 0,
        V2 = 1,
    }
    impl Version {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Version::V1 => "V1",
                Version::V2 => "V2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "V1" => Some(Self::V1),
                "V2" => Some(Self::V2),
                _ => None,
            }
        }
    }
}
