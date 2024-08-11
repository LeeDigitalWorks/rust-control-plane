// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpListenerConfig {
    #[prost(string, tag = "1")]
    pub udp_listener_name: ::prost::alloc::string::String,
    #[prost(oneof = "udp_listener_config::ConfigType", tags = "2, 3")]
    pub config_type: ::core::option::Option<udp_listener_config::ConfigType>,
}
/// Nested message and enum types in `UdpListenerConfig`.
pub mod udp_listener_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "2")]
        Config(super::super::super::super::super::google::protobuf::Struct),
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ActiveRawUdpListenerConfig {}
