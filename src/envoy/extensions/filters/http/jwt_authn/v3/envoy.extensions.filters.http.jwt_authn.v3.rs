// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtProvider {
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "19")]
    pub subjects: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    #[prost(bool, tag = "20")]
    pub require_expiration: bool,
    #[prost(message, optional, tag = "21")]
    pub max_lifetime: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(bool, tag = "5")]
    pub forward: bool,
    #[prost(message, repeated, tag = "6")]
    pub from_headers: ::prost::alloc::vec::Vec<JwtHeader>,
    #[prost(string, repeated, tag = "7")]
    pub from_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "13")]
    pub from_cookies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "8")]
    pub forward_payload_header: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub pad_forward_payload_header: bool,
    #[prost(string, tag = "9")]
    pub payload_in_metadata: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "18")]
    pub normalize_payload_in_metadata: ::core::option::Option<
        jwt_provider::NormalizePayload,
    >,
    #[prost(string, tag = "14")]
    pub header_in_metadata: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub failed_status_in_metadata: ::prost::alloc::string::String,
    #[prost(uint32, tag = "10")]
    pub clock_skew_seconds: u32,
    #[prost(message, optional, tag = "12")]
    pub jwt_cache_config: ::core::option::Option<JwtCacheConfig>,
    #[prost(message, repeated, tag = "15")]
    pub claim_to_headers: ::prost::alloc::vec::Vec<JwtClaimToHeader>,
    #[prost(bool, tag = "17")]
    pub clear_route_cache: bool,
    #[prost(oneof = "jwt_provider::JwksSourceSpecifier", tags = "3, 4")]
    pub jwks_source_specifier: ::core::option::Option<jwt_provider::JwksSourceSpecifier>,
}
/// Nested message and enum types in `JwtProvider`.
pub mod jwt_provider {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NormalizePayload {
        #[prost(string, repeated, tag = "1")]
        pub space_delimited_claims: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JwksSourceSpecifier {
        #[prost(message, tag = "3")]
        RemoteJwks(super::RemoteJwks),
        #[prost(message, tag = "4")]
        LocalJwks(
            super::super::super::super::super::super::config::core::v3::DataSource,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct JwtCacheConfig {
    #[prost(uint32, tag = "1")]
    pub jwt_cache_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteJwks {
    #[prost(message, optional, tag = "1")]
    pub http_uri: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpUri,
    >,
    #[prost(message, optional, tag = "2")]
    pub cache_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub async_fetch: ::core::option::Option<JwksAsyncFetch>,
    #[prost(message, optional, tag = "4")]
    pub retry_policy: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RetryPolicy,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct JwksAsyncFetch {
    #[prost(bool, tag = "1")]
    pub fast_listener: bool,
    #[prost(message, optional, tag = "2")]
    pub failed_refetch_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtHeader {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value_prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderWithAudiences {
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirement {
    #[prost(oneof = "jwt_requirement::RequiresType", tags = "1, 2, 3, 4, 5, 6")]
    pub requires_type: ::core::option::Option<jwt_requirement::RequiresType>,
}
/// Nested message and enum types in `JwtRequirement`.
pub mod jwt_requirement {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequiresType {
        #[prost(string, tag = "1")]
        ProviderName(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        ProviderAndAudiences(super::ProviderWithAudiences),
        #[prost(message, tag = "3")]
        RequiresAny(super::JwtRequirementOrList),
        #[prost(message, tag = "4")]
        RequiresAll(super::JwtRequirementAndList),
        #[prost(message, tag = "5")]
        AllowMissingOrFailed(
            super::super::super::super::super::super::super::google::protobuf::Empty,
        ),
        #[prost(message, tag = "6")]
        AllowMissing(
            super::super::super::super::super::super::super::google::protobuf::Empty,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirementOrList {
    #[prost(message, repeated, tag = "1")]
    pub requirements: ::prost::alloc::vec::Vec<JwtRequirement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirementAndList {
    #[prost(message, repeated, tag = "1")]
    pub requirements: ::prost::alloc::vec::Vec<JwtRequirement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequirementRule {
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<
        super::super::super::super::super::config::route::v3::RouteMatch,
    >,
    #[prost(oneof = "requirement_rule::RequirementType", tags = "2, 3")]
    pub requirement_type: ::core::option::Option<requirement_rule::RequirementType>,
}
/// Nested message and enum types in `RequirementRule`.
pub mod requirement_rule {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequirementType {
        #[prost(message, tag = "2")]
        Requires(super::JwtRequirement),
        #[prost(string, tag = "3")]
        RequirementName(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateRule {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "3")]
    pub requires: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtRequirement,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtAuthentication {
    #[prost(map = "string, message", tag = "1")]
    pub providers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtProvider,
    >,
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<RequirementRule>,
    #[prost(message, optional, tag = "3")]
    pub filter_state_rules: ::core::option::Option<FilterStateRule>,
    #[prost(bool, tag = "4")]
    pub bypass_cors_preflight: bool,
    #[prost(map = "string, message", tag = "5")]
    pub requirement_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtRequirement,
    >,
    #[prost(bool, tag = "6")]
    pub strip_failure_response: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(oneof = "per_route_config::RequirementSpecifier", tags = "1, 2")]
    pub requirement_specifier: ::core::option::Option<
        per_route_config::RequirementSpecifier,
    >,
}
/// Nested message and enum types in `PerRouteConfig`.
pub mod per_route_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequirementSpecifier {
        #[prost(bool, tag = "1")]
        Disabled(bool),
        #[prost(string, tag = "2")]
        RequirementName(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtClaimToHeader {
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub claim_name: ::prost::alloc::string::String,
}
