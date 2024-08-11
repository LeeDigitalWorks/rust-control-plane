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
pub struct DnsResolverOptions {
    #[prost(bool, tag = "1")]
    pub use_tcp_for_dns_lookups: bool,
    #[prost(bool, tag = "2")]
    pub no_default_search_domain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsResolutionConfig {
    #[prost(message, repeated, tag = "1")]
    pub resolvers: ::prost::alloc::vec::Vec<Address>,
    #[prost(message, optional, tag = "2")]
    pub dns_resolver_options: ::core::option::Option<DnsResolverOptions>,
}
