// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Router {
    #[prost(message, optional, tag = "1")]
    pub close_downstream_on_upstream_error: ::core::option::Option<
        super::super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
