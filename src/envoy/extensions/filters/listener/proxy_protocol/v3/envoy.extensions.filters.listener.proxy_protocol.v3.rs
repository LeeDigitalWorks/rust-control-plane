// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<proxy_protocol::Rule>,
    #[prost(bool, tag = "2")]
    pub allow_requests_without_proxy_protocol: bool,
    #[prost(message, optional, tag = "3")]
    pub pass_through_tlvs: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ProxyProtocolPassThroughTlVs,
    >,
    #[prost(
        enumeration = "super::super::super::super::super::config::core::v3::proxy_protocol_config::Version",
        repeated,
        tag = "4"
    )]
    pub disallowed_versions: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "5")]
    pub stat_prefix: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValuePair {
        #[prost(string, tag = "1")]
        pub metadata_namespace: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub key: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(uint32, tag = "1")]
        pub tlv_type: u32,
        #[prost(message, optional, tag = "2")]
        pub on_tlv_present: ::core::option::Option<KeyValuePair>,
    }
}
