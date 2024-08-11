// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsrfPolicy {
    #[prost(message, optional, tag = "1")]
    pub filter_enabled: ::core::option::Option<
        super::super::super::super::super::api::v2::core::RuntimeFractionalPercent,
    >,
    #[prost(message, optional, tag = "2")]
    pub shadow_enabled: ::core::option::Option<
        super::super::super::super::super::api::v2::core::RuntimeFractionalPercent,
    >,
    #[prost(message, repeated, tag = "3")]
    pub additional_origins: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::matcher::StringMatcher,
    >,
}
