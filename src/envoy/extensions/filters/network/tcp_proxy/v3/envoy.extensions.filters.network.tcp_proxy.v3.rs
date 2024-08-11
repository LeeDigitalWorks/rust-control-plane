// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProxy {
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub on_demand: ::core::option::Option<tcp_proxy::OnDemand>,
    #[prost(message, optional, tag = "9")]
    pub metadata_match: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Metadata,
    >,
    #[prost(message, optional, tag = "8")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub downstream_idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "4")]
    pub upstream_idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, repeated, tag = "5")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    #[prost(message, optional, tag = "7")]
    pub max_connect_attempts: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, repeated, tag = "11")]
    pub hash_policy: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::v3::HashPolicy,
    >,
    #[prost(message, optional, tag = "12")]
    pub tunneling_config: ::core::option::Option<tcp_proxy::TunnelingConfig>,
    #[prost(message, optional, tag = "13")]
    pub max_downstream_connection_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[deprecated]
    #[prost(message, optional, tag = "15")]
    pub access_log_flush_interval: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[deprecated]
    #[prost(bool, tag = "16")]
    pub flush_access_log_on_connected: bool,
    #[prost(message, optional, tag = "17")]
    pub access_log_options: ::core::option::Option<tcp_proxy::TcpAccessLogOptions>,
    #[prost(oneof = "tcp_proxy::ClusterSpecifier", tags = "2, 10")]
    pub cluster_specifier: ::core::option::Option<tcp_proxy::ClusterSpecifier>,
}
/// Nested message and enum types in `TcpProxy`.
pub mod tcp_proxy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeightedCluster {
        #[prost(message, repeated, tag = "1")]
        pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
    }
    /// Nested message and enum types in `WeightedCluster`.
    pub mod weighted_cluster {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ClusterWeight {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(uint32, tag = "2")]
            pub weight: u32,
            #[prost(message, optional, tag = "3")]
            pub metadata_match: ::core::option::Option<
                super::super::super::super::super::super::super::config::core::v3::Metadata,
            >,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TunnelingConfig {
        #[prost(string, tag = "1")]
        pub hostname: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub use_post: bool,
        #[prost(message, repeated, tag = "3")]
        pub headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::HeaderValueOption,
        >,
        #[prost(bool, tag = "4")]
        pub propagate_response_headers: bool,
        #[prost(string, tag = "5")]
        pub post_path: ::prost::alloc::string::String,
        #[prost(bool, tag = "6")]
        pub propagate_response_trailers: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnDemand {
        #[prost(message, optional, tag = "1")]
        pub odcds_config: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::ConfigSource,
        >,
        #[prost(string, tag = "2")]
        pub resources_locator: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct TcpAccessLogOptions {
        #[prost(message, optional, tag = "1")]
        pub access_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        #[prost(bool, tag = "2")]
        pub flush_access_log_on_connected: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        #[prost(string, tag = "2")]
        Cluster(::prost::alloc::string::String),
        #[prost(message, tag = "10")]
        WeightedClusters(WeightedCluster),
    }
}
