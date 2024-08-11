// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    #[prost(message, optional, tag = "1")]
    pub pass_through_mode: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "3")]
    pub cache_time: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(map = "string, message", tag = "4")]
    pub cluster_min_healthy_percentages: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::super::r#type::v3::Percent,
    >,
    #[prost(message, repeated, tag = "5")]
    pub headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
}
