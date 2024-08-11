// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RedisClusterConfig {
    #[prost(message, optional, tag = "1")]
    pub cluster_refresh_rate: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub cluster_refresh_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub redirect_refresh_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "4")]
    pub redirect_refresh_threshold: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(uint32, tag = "5")]
    pub failure_refresh_threshold: u32,
    #[prost(uint32, tag = "6")]
    pub host_degraded_refresh_threshold: u32,
}
