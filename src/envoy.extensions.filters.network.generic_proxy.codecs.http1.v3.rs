// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Http1CodecConfig {
    #[prost(message, optional, tag = "1")]
    pub single_frame_mode: ::core::option::Option<
        super::super::super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "2")]
    pub max_buffer_size: ::core::option::Option<
        super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
