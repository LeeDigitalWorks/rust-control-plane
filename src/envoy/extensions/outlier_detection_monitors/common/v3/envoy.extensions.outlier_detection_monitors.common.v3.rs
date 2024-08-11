// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpErrors {
    #[prost(message, optional, tag = "1")]
    pub range: ::core::option::Option<
        super::super::super::super::r#type::v3::Int32Range,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LocalOriginErrors {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DatabaseErrors {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorBuckets {
    #[prost(message, repeated, tag = "1")]
    pub http_errors: ::prost::alloc::vec::Vec<HttpErrors>,
    #[prost(message, repeated, tag = "2")]
    pub local_origin_errors: ::prost::alloc::vec::Vec<LocalOriginErrors>,
    #[prost(message, repeated, tag = "3")]
    pub database_errors: ::prost::alloc::vec::Vec<DatabaseErrors>,
}
