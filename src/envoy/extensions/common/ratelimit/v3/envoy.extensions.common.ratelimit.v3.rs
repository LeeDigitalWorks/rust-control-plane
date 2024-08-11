// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitDescriptor {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<rate_limit_descriptor::Entry>,
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<rate_limit_descriptor::RateLimitOverride>,
}
/// Nested message and enum types in `RateLimitDescriptor`.
pub mod rate_limit_descriptor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RateLimitOverride {
        #[prost(uint32, tag = "1")]
        pub requests_per_unit: u32,
        #[prost(
            enumeration = "super::super::super::super::super::r#type::v3::RateLimitUnit",
            tag = "2"
        )]
        pub unit: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimitDescriptor {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<rate_limit_descriptor::Entry>,
    #[prost(message, optional, tag = "2")]
    pub token_bucket: ::core::option::Option<
        super::super::super::super::r#type::v3::TokenBucket,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LocalClusterRateLimit {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XRateLimitHeadersRfcVersion {
    Off = 0,
    DraftVersion03 = 1,
}
impl XRateLimitHeadersRfcVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            XRateLimitHeadersRfcVersion::Off => "OFF",
            XRateLimitHeadersRfcVersion::DraftVersion03 => "DRAFT_VERSION_03",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFF" => Some(Self::Off),
            "DRAFT_VERSION_03" => Some(Self::DraftVersion03),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VhRateLimitsOptions {
    Override = 0,
    Include = 1,
    Ignore = 2,
}
impl VhRateLimitsOptions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VhRateLimitsOptions::Override => "OVERRIDE",
            VhRateLimitsOptions::Include => "INCLUDE",
            VhRateLimitsOptions::Ignore => "IGNORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OVERRIDE" => Some(Self::Override),
            "INCLUDE" => Some(Self::Include),
            "IGNORE" => Some(Self::Ignore),
            _ => None,
        }
    }
}
