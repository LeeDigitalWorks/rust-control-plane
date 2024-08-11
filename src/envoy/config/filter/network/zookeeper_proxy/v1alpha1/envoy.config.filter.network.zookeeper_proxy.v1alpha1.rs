// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZooKeeperProxy {
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_log: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub max_packet_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
