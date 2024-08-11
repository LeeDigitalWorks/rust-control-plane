// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryRequest {
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<core::Node>,
    #[prost(string, repeated, tag = "3")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub error_detail: ::core::option::Option<super::super::super::google::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryResponse {
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<super::super::super::google::protobuf::Any>,
    #[prost(bool, tag = "3")]
    pub canary: bool,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub control_plane: ::core::option::Option<core::ControlPlane>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryRequest {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<core::Node>,
    #[prost(string, tag = "2")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub resource_names_subscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "4")]
    pub resource_names_unsubscribe: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "5")]
    pub initial_resource_versions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "6")]
    pub response_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub error_detail: ::core::option::Option<super::super::super::google::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryResponse {
    #[prost(string, tag = "1")]
    pub system_version_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    #[prost(string, tag = "4")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub removed_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
