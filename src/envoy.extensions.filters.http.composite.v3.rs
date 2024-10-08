// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Composite {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub config_discovery: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ExtensionConfigSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFilterAction {
    #[prost(message, optional, tag = "1")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "2")]
    pub dynamic_config: ::core::option::Option<DynamicConfig>,
    #[prost(message, optional, tag = "3")]
    pub sample_percent: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
}
