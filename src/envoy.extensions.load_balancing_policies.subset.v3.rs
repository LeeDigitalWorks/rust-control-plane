// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subset {
    #[prost(enumeration = "subset::LbSubsetFallbackPolicy", tag = "1")]
    pub fallback_policy: i32,
    #[prost(message, optional, tag = "2")]
    pub default_subset: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Struct,
    >,
    #[prost(message, repeated, tag = "3")]
    pub subset_selectors: ::prost::alloc::vec::Vec<subset::LbSubsetSelector>,
    #[prost(bool, tag = "10")]
    pub allow_redundant_keys: bool,
    #[prost(bool, tag = "4")]
    pub locality_weight_aware: bool,
    #[prost(bool, tag = "5")]
    pub scale_locality_weight: bool,
    #[prost(bool, tag = "6")]
    pub panic_mode_any: bool,
    #[prost(bool, tag = "7")]
    pub list_as_any: bool,
    #[prost(enumeration = "subset::LbSubsetMetadataFallbackPolicy", tag = "8")]
    pub metadata_fallback_policy: i32,
    #[prost(message, optional, tag = "9")]
    pub subset_lb_policy: ::core::option::Option<
        super::super::super::super::config::cluster::v3::LoadBalancingPolicy,
    >,
}
/// Nested message and enum types in `Subset`.
pub mod subset {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LbSubsetSelector {
        #[prost(string, repeated, tag = "1")]
        pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bool, tag = "4")]
        pub single_host_per_subset: bool,
        #[prost(
            enumeration = "lb_subset_selector::LbSubsetSelectorFallbackPolicy",
            tag = "2"
        )]
        pub fallback_policy: i32,
        #[prost(string, repeated, tag = "3")]
        pub fallback_keys_subset: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// Nested message and enum types in `LbSubsetSelector`.
    pub mod lb_subset_selector {
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
        pub enum LbSubsetSelectorFallbackPolicy {
            NotDefined = 0,
            NoFallback = 1,
            AnyEndpoint = 2,
            DefaultSubset = 3,
            KeysSubset = 4,
        }
        impl LbSubsetSelectorFallbackPolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    LbSubsetSelectorFallbackPolicy::NotDefined => "NOT_DEFINED",
                    LbSubsetSelectorFallbackPolicy::NoFallback => "NO_FALLBACK",
                    LbSubsetSelectorFallbackPolicy::AnyEndpoint => "ANY_ENDPOINT",
                    LbSubsetSelectorFallbackPolicy::DefaultSubset => "DEFAULT_SUBSET",
                    LbSubsetSelectorFallbackPolicy::KeysSubset => "KEYS_SUBSET",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NOT_DEFINED" => Some(Self::NotDefined),
                    "NO_FALLBACK" => Some(Self::NoFallback),
                    "ANY_ENDPOINT" => Some(Self::AnyEndpoint),
                    "DEFAULT_SUBSET" => Some(Self::DefaultSubset),
                    "KEYS_SUBSET" => Some(Self::KeysSubset),
                    _ => None,
                }
            }
        }
    }
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
    pub enum LbSubsetFallbackPolicy {
        NoFallback = 0,
        AnyEndpoint = 1,
        DefaultSubset = 2,
    }
    impl LbSubsetFallbackPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LbSubsetFallbackPolicy::NoFallback => "NO_FALLBACK",
                LbSubsetFallbackPolicy::AnyEndpoint => "ANY_ENDPOINT",
                LbSubsetFallbackPolicy::DefaultSubset => "DEFAULT_SUBSET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_FALLBACK" => Some(Self::NoFallback),
                "ANY_ENDPOINT" => Some(Self::AnyEndpoint),
                "DEFAULT_SUBSET" => Some(Self::DefaultSubset),
                _ => None,
            }
        }
    }
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
    pub enum LbSubsetMetadataFallbackPolicy {
        MetadataNoFallback = 0,
        FallbackList = 1,
    }
    impl LbSubsetMetadataFallbackPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LbSubsetMetadataFallbackPolicy::MetadataNoFallback => {
                    "METADATA_NO_FALLBACK"
                }
                LbSubsetMetadataFallbackPolicy::FallbackList => "FALLBACK_LIST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METADATA_NO_FALLBACK" => Some(Self::MetadataNoFallback),
                "FALLBACK_LIST" => Some(Self::FallbackList),
                _ => None,
            }
        }
    }
}
