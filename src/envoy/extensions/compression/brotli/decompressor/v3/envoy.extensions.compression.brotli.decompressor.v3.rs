// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Brotli {
    #[prost(bool, tag = "1")]
    pub disable_ring_buffer_reallocation: bool,
    #[prost(message, optional, tag = "2")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
