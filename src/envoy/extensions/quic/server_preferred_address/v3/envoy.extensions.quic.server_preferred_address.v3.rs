// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedServerPreferredAddressConfig {
    #[prost(string, tag = "1")]
    pub ipv4_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub ipv4_config: ::core::option::Option<
        fixed_server_preferred_address_config::AddressFamilyConfig,
    >,
    #[prost(string, tag = "2")]
    pub ipv6_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub ipv6_config: ::core::option::Option<
        fixed_server_preferred_address_config::AddressFamilyConfig,
    >,
}
/// Nested message and enum types in `FixedServerPreferredAddressConfig`.
pub mod fixed_server_preferred_address_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressFamilyConfig {
        #[prost(message, optional, tag = "1")]
        pub address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::SocketAddress,
        >,
        #[prost(message, optional, tag = "2")]
        pub dnat_address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::SocketAddress,
        >,
    }
}
