// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    #[prost(message, optional, tag = "6")]
    pub matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    #[prost(message, optional, tag = "2")]
    pub shadow_rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    #[prost(message, optional, tag = "7")]
    pub shadow_matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    #[prost(string, tag = "5")]
    pub shadow_rules_stat_prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(enumeration = "rbac::EnforcementType", tag = "4")]
    pub enforcement_type: i32,
}
/// Nested message and enum types in `RBAC`.
pub mod rbac {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EnforcementType {
        OneTimeOnFirstByte = 0,
        Continuous = 1,
    }
    impl EnforcementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnforcementType::OneTimeOnFirstByte => "ONE_TIME_ON_FIRST_BYTE",
                EnforcementType::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ONE_TIME_ON_FIRST_BYTE" => Some(Self::OneTimeOnFirstByte),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
}
