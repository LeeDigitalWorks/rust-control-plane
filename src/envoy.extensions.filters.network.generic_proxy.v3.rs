// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualHost {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub routes: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub routes: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    #[prost(message, repeated, tag = "3")]
    pub virtual_hosts: ::prost::alloc::vec::Vec<VirtualHost>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericProxy {
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub codec_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, repeated, tag = "5")]
    pub filters: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "6")]
    pub tracing: ::core::option::Option<
        super::super::http_connection_manager::v3::http_connection_manager::Tracing,
    >,
    #[prost(message, repeated, tag = "7")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    #[prost(oneof = "generic_proxy::RouteSpecifier", tags = "3, 4")]
    pub route_specifier: ::core::option::Option<generic_proxy::RouteSpecifier>,
}
/// Nested message and enum types in `GenericProxy`.
pub mod generic_proxy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        #[prost(message, tag = "3")]
        GenericRds(super::GenericRds),
        #[prost(message, tag = "4")]
        RouteConfig(super::RouteConfiguration),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericRds {
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    #[prost(string, tag = "2")]
    pub route_config_name: ::prost::alloc::string::String,
}
