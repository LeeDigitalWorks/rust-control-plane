// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsIamConfig {
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileBasedMetadataConfig {
    #[prost(message, optional, tag = "1")]
    pub secret_data: ::core::option::Option<super::super::core::v3::DataSource>,
    #[prost(string, tag = "2")]
    pub header_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub header_prefix: ::prost::alloc::string::String,
}
