// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zstd {
    #[prost(message, repeated, tag = "1")]
    pub dictionaries: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "2")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
