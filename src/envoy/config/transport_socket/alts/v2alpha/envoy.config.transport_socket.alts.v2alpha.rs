// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alts {
    #[prost(string, tag = "1")]
    pub handshaker_service: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub peer_service_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
