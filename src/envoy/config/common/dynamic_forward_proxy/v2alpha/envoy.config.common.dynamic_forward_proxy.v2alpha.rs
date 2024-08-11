// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsCacheConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::super::api::v2::cluster::DnsLookupFamily",
        tag = "2"
    )]
    pub dns_lookup_family: i32,
    #[prost(message, optional, tag = "3")]
    pub dns_refresh_rate: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "4")]
    pub host_ttl: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "5")]
    pub max_hosts: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "6")]
    pub dns_failure_refresh_rate: ::core::option::Option<
        super::super::super::super::api::v2::cluster::RefreshRate,
    >,
}
