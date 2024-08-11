// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalUpstreamTransport {
    #[prost(message, repeated, tag = "1")]
    pub passthrough_metadata: ::prost::alloc::vec::Vec<
        internal_upstream_transport::MetadataValueSource,
    >,
    #[prost(message, optional, tag = "3")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
/// Nested message and enum types in `InternalUpstreamTransport`.
pub mod internal_upstream_transport {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataValueSource {
        #[prost(message, optional, tag = "1")]
        pub kind: ::core::option::Option<
            super::super::super::super::super::r#type::metadata::v3::MetadataKind,
        >,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
}
