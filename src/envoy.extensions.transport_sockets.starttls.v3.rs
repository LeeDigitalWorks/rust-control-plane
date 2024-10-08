// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTlsConfig {
    #[prost(message, optional, tag = "1")]
    pub cleartext_socket_config: ::core::option::Option<
        super::super::raw_buffer::v3::RawBuffer,
    >,
    #[prost(message, optional, tag = "2")]
    pub tls_socket_config: ::core::option::Option<
        super::super::tls::v3::DownstreamTlsContext,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamStartTlsConfig {
    #[prost(message, optional, tag = "1")]
    pub cleartext_socket_config: ::core::option::Option<
        super::super::raw_buffer::v3::RawBuffer,
    >,
    #[prost(message, optional, tag = "2")]
    pub tls_socket_config: ::core::option::Option<
        super::super::tls::v3::UpstreamTlsContext,
    >,
}
