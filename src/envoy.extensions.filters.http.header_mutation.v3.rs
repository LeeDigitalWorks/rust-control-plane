// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutations {
    #[prost(message, repeated, tag = "1")]
    pub request_mutations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutation,
    >,
    #[prost(message, repeated, tag = "2")]
    pub response_mutations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutation,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutationPerRoute {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
    #[prost(bool, tag = "2")]
    pub most_specific_header_mutations_wins: bool,
}
