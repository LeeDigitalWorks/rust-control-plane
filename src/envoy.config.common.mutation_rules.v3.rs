// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutationRules {
    #[prost(message, optional, tag = "1")]
    pub allow_all_routing: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "2")]
    pub allow_envoy: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "3")]
    pub disallow_system: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "4")]
    pub disallow_all: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "5")]
    pub allow_expression: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::RegexMatcher,
    >,
    #[prost(message, optional, tag = "6")]
    pub disallow_expression: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::RegexMatcher,
    >,
    #[prost(message, optional, tag = "7")]
    pub disallow_is_error: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(oneof = "header_mutation::Action", tags = "1, 2")]
    pub action: ::core::option::Option<header_mutation::Action>,
}
/// Nested message and enum types in `HeaderMutation`.
pub mod header_mutation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(string, tag = "1")]
        Remove(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Append(super::super::super::super::core::v3::HeaderValueOption),
    }
}
