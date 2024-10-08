// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitQuotaFilterConfig {
    #[prost(message, optional, tag = "1")]
    pub rlqs_server: ::core::option::Option<
        super::super::super::super::super::config::core::v3::GrpcService,
    >,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub bucket_matchers: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    #[prost(message, optional, tag = "4")]
    pub filter_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    #[prost(message, optional, tag = "5")]
    pub filter_enforced: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    #[prost(message, repeated, tag = "6")]
    pub request_headers_to_add_when_not_enforced: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitQuotaOverride {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub bucket_matchers: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitQuotaBucketSettings {
    #[prost(message, optional, tag = "1")]
    pub bucket_id_builder: ::core::option::Option<
        rate_limit_quota_bucket_settings::BucketIdBuilder,
    >,
    #[prost(message, optional, tag = "2")]
    pub reporting_interval: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub deny_response_settings: ::core::option::Option<
        rate_limit_quota_bucket_settings::DenyResponseSettings,
    >,
    #[prost(message, optional, tag = "4")]
    pub no_assignment_behavior: ::core::option::Option<
        rate_limit_quota_bucket_settings::NoAssignmentBehavior,
    >,
    #[prost(message, optional, tag = "5")]
    pub expired_assignment_behavior: ::core::option::Option<
        rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior,
    >,
}
/// Nested message and enum types in `RateLimitQuotaBucketSettings`.
pub mod rate_limit_quota_bucket_settings {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct NoAssignmentBehavior {
        #[prost(oneof = "no_assignment_behavior::NoAssignmentBehavior", tags = "1")]
        pub no_assignment_behavior: ::core::option::Option<
            no_assignment_behavior::NoAssignmentBehavior,
        >,
    }
    /// Nested message and enum types in `NoAssignmentBehavior`.
    pub mod no_assignment_behavior {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum NoAssignmentBehavior {
            #[prost(message, tag = "1")]
            FallbackRateLimit(
                super::super::super::super::super::super::super::r#type::v3::RateLimitStrategy,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ExpiredAssignmentBehavior {
        #[prost(message, optional, tag = "1")]
        pub expired_assignment_behavior_timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        #[prost(
            oneof = "expired_assignment_behavior::ExpiredAssignmentBehavior",
            tags = "2, 3"
        )]
        pub expired_assignment_behavior: ::core::option::Option<
            expired_assignment_behavior::ExpiredAssignmentBehavior,
        >,
    }
    /// Nested message and enum types in `ExpiredAssignmentBehavior`.
    pub mod expired_assignment_behavior {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct ReuseLastAssignment {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum ExpiredAssignmentBehavior {
            #[prost(message, tag = "2")]
            FallbackRateLimit(
                super::super::super::super::super::super::super::r#type::v3::RateLimitStrategy,
            ),
            #[prost(message, tag = "3")]
            ReuseLastAssignment(ReuseLastAssignment),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DenyResponseSettings {
        #[prost(message, optional, tag = "1")]
        pub http_status: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::HttpStatus,
        >,
        #[prost(message, optional, tag = "2")]
        pub http_body: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::BytesValue,
        >,
        #[prost(message, optional, tag = "3")]
        pub grpc_status: ::core::option::Option<
            super::super::super::super::super::super::super::google::rpc::Status,
        >,
        #[prost(message, repeated, tag = "4")]
        pub response_headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::HeaderValueOption,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BucketIdBuilder {
        #[prost(map = "string, message", tag = "1")]
        pub bucket_id_builder: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            bucket_id_builder::ValueBuilder,
        >,
    }
    /// Nested message and enum types in `BucketIdBuilder`.
    pub mod bucket_id_builder {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ValueBuilder {
            #[prost(oneof = "value_builder::ValueSpecifier", tags = "1, 2")]
            pub value_specifier: ::core::option::Option<value_builder::ValueSpecifier>,
        }
        /// Nested message and enum types in `ValueBuilder`.
        pub mod value_builder {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum ValueSpecifier {
                #[prost(string, tag = "1")]
                StringValue(::prost::alloc::string::String),
                #[prost(message, tag = "2")]
                CustomValue(
                    super::super::super::super::super::super::super::super::config::core::v3::TypedExtensionConfig,
                ),
            }
        }
    }
}
