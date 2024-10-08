// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpProxyConfig {
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(bool, tag = "4")]
    pub use_original_src_ip: bool,
    #[prost(message, repeated, tag = "5")]
    pub hash_policies: ::prost::alloc::vec::Vec<udp_proxy_config::HashPolicy>,
    #[prost(message, optional, tag = "6")]
    pub upstream_socket_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::UdpSocketConfig,
    >,
    #[prost(bool, tag = "7")]
    pub use_per_packet_load_balancing: bool,
    #[prost(message, repeated, tag = "8")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    #[prost(message, repeated, tag = "10")]
    pub proxy_access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    #[prost(message, repeated, tag = "11")]
    pub session_filters: ::prost::alloc::vec::Vec<udp_proxy_config::SessionFilter>,
    #[prost(message, optional, tag = "12")]
    pub tunneling_config: ::core::option::Option<udp_proxy_config::UdpTunnelingConfig>,
    #[prost(message, optional, tag = "13")]
    pub access_log_options: ::core::option::Option<
        udp_proxy_config::UdpAccessLogOptions,
    >,
    #[prost(oneof = "udp_proxy_config::RouteSpecifier", tags = "2, 9")]
    pub route_specifier: ::core::option::Option<udp_proxy_config::RouteSpecifier>,
}
/// Nested message and enum types in `UdpProxyConfig`.
pub mod udp_proxy_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashPolicy {
        #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2")]
        pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
    }
    /// Nested message and enum types in `HashPolicy`.
    pub mod hash_policy {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicySpecifier {
            #[prost(bool, tag = "1")]
            SourceIp(bool),
            #[prost(string, tag = "2")]
            Key(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionFilter {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "session_filter::ConfigType", tags = "2")]
        pub config_type: ::core::option::Option<session_filter::ConfigType>,
    }
    /// Nested message and enum types in `SessionFilter`.
    pub mod session_filter {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "2")]
            TypedConfig(
                super::super::super::super::super::super::super::super::google::protobuf::Any,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UdpTunnelingConfig {
        #[prost(string, tag = "1")]
        pub proxy_host: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub proxy_port: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        #[prost(string, tag = "3")]
        pub target_host: ::prost::alloc::string::String,
        #[prost(uint32, tag = "4")]
        pub default_target_port: u32,
        #[prost(bool, tag = "5")]
        pub use_post: bool,
        #[prost(string, tag = "6")]
        pub post_path: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "7")]
        pub retry_options: ::core::option::Option<udp_tunneling_config::RetryOptions>,
        #[prost(message, repeated, tag = "8")]
        pub headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::HeaderValueOption,
        >,
        #[prost(message, optional, tag = "9")]
        pub buffer_options: ::core::option::Option<udp_tunneling_config::BufferOptions>,
        #[prost(bool, tag = "10")]
        pub propagate_response_headers: bool,
        #[prost(bool, tag = "11")]
        pub propagate_response_trailers: bool,
    }
    /// Nested message and enum types in `UdpTunnelingConfig`.
    pub mod udp_tunneling_config {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct BufferOptions {
            #[prost(message, optional, tag = "1")]
            pub max_buffered_datagrams: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
            #[prost(message, optional, tag = "2")]
            pub max_buffered_bytes: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt64Value,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct RetryOptions {
            #[prost(message, optional, tag = "1")]
            pub max_connect_attempts: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UdpAccessLogOptions {
        #[prost(message, optional, tag = "1")]
        pub access_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        #[prost(bool, tag = "2")]
        pub flush_access_log_on_tunnel_connected: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        #[prost(string, tag = "2")]
        Cluster(::prost::alloc::string::String),
        #[prost(message, tag = "9")]
        Matcher(
            super::super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
        ),
    }
}
