// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TraceConfig {
    #[prost(int64, tag = "4")]
    pub max_number_of_attributes: i64,
    #[prost(int64, tag = "5")]
    pub max_number_of_annotations: i64,
    #[prost(int64, tag = "6")]
    pub max_number_of_message_events: i64,
    #[prost(int64, tag = "7")]
    pub max_number_of_links: i64,
    #[prost(oneof = "trace_config::Sampler", tags = "1, 2, 3")]
    pub sampler: ::core::option::Option<trace_config::Sampler>,
}
/// Nested message and enum types in `TraceConfig`.
pub mod trace_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Sampler {
        #[prost(message, tag = "1")]
        ProbabilitySampler(super::ProbabilitySampler),
        #[prost(message, tag = "2")]
        ConstantSampler(super::ConstantSampler),
        #[prost(message, tag = "3")]
        RateLimitingSampler(super::RateLimitingSampler),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ProbabilitySampler {
    #[prost(double, tag = "1")]
    pub sampling_probability: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConstantSampler {
    #[prost(enumeration = "constant_sampler::ConstantDecision", tag = "1")]
    pub decision: i32,
}
/// Nested message and enum types in `ConstantSampler`.
pub mod constant_sampler {
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
    pub enum ConstantDecision {
        AlwaysOff = 0,
        AlwaysOn = 1,
        AlwaysParent = 2,
    }
    impl ConstantDecision {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConstantDecision::AlwaysOff => "ALWAYS_OFF",
                ConstantDecision::AlwaysOn => "ALWAYS_ON",
                ConstantDecision::AlwaysParent => "ALWAYS_PARENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALWAYS_OFF" => Some(Self::AlwaysOff),
                "ALWAYS_ON" => Some(Self::AlwaysOn),
                "ALWAYS_PARENT" => Some(Self::AlwaysParent),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RateLimitingSampler {
    #[prost(int64, tag = "1")]
    pub qps: i64,
}
