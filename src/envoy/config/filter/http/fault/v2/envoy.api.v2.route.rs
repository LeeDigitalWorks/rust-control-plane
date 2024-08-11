// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualHost {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    #[prost(enumeration = "virtual_host::TlsRequirementType", tag = "4")]
    pub require_tls: i32,
    #[prost(message, repeated, tag = "5")]
    pub virtual_clusters: ::prost::alloc::vec::Vec<VirtualCluster>,
    #[prost(message, repeated, tag = "6")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    #[prost(message, repeated, tag = "7")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<super::core::HeaderValueOption>,
    #[prost(string, repeated, tag = "13")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::core::HeaderValueOption,
    >,
    #[prost(string, repeated, tag = "11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "8")]
    pub cors: ::core::option::Option<CorsPolicy>,
    #[prost(map = "string, message", tag = "12")]
    pub per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
    #[prost(map = "string, message", tag = "15")]
    pub typed_per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    #[prost(bool, tag = "14")]
    pub include_request_attempt_count: bool,
    #[prost(bool, tag = "19")]
    pub include_attempt_count_in_response: bool,
    #[prost(message, optional, tag = "16")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    #[prost(message, optional, tag = "20")]
    pub retry_policy_typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    #[prost(message, optional, tag = "17")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    #[prost(message, optional, tag = "18")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `VirtualHost`.
pub mod virtual_host {
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
    pub enum TlsRequirementType {
        None = 0,
        ExternalOnly = 1,
        All = 2,
    }
    impl TlsRequirementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsRequirementType::None => "NONE",
                TlsRequirementType::ExternalOnly => "EXTERNAL_ONLY",
                TlsRequirementType::All => "ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "EXTERNAL_ONLY" => Some(Self::ExternalOnly),
                "ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterAction {
    #[prost(message, optional, tag = "1")]
    pub action: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<RouteMatch>,
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<super::core::Metadata>,
    #[prost(message, optional, tag = "5")]
    pub decorator: ::core::option::Option<Decorator>,
    #[prost(map = "string, message", tag = "8")]
    pub per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Struct,
    >,
    #[prost(map = "string, message", tag = "13")]
    pub typed_per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    #[prost(message, repeated, tag = "9")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<super::core::HeaderValueOption>,
    #[prost(string, repeated, tag = "12")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::core::HeaderValueOption,
    >,
    #[prost(string, repeated, tag = "11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "15")]
    pub tracing: ::core::option::Option<Tracing>,
    #[prost(message, optional, tag = "16")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(oneof = "route::Action", tags = "2, 3, 7, 17")]
    pub action: ::core::option::Option<route::Action>,
}
/// Nested message and enum types in `Route`.
pub mod route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "2")]
        Route(super::RouteAction),
        #[prost(message, tag = "3")]
        Redirect(super::RedirectAction),
        #[prost(message, tag = "7")]
        DirectResponse(super::DirectResponseAction),
        #[prost(message, tag = "17")]
        FilterAction(super::FilterAction),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedCluster {
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
    #[prost(message, optional, tag = "3")]
    pub total_weight: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(string, tag = "2")]
    pub runtime_key_prefix: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WeightedCluster`.
pub mod weighted_cluster {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterWeight {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub weight: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        #[prost(message, optional, tag = "3")]
        pub metadata_match: ::core::option::Option<super::super::core::Metadata>,
        #[prost(message, repeated, tag = "4")]
        pub request_headers_to_add: ::prost::alloc::vec::Vec<
            super::super::core::HeaderValueOption,
        >,
        #[prost(string, repeated, tag = "9")]
        pub request_headers_to_remove: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(message, repeated, tag = "5")]
        pub response_headers_to_add: ::prost::alloc::vec::Vec<
            super::super::core::HeaderValueOption,
        >,
        #[prost(string, repeated, tag = "6")]
        pub response_headers_to_remove: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(map = "string, message", tag = "8")]
        pub per_filter_config: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::super::super::google::protobuf::Struct,
        >,
        #[prost(map = "string, message", tag = "10")]
        pub typed_per_filter_config: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::super::super::google::protobuf::Any,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatch {
    #[prost(message, optional, tag = "4")]
    pub case_sensitive: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "9")]
    pub runtime_fraction: ::core::option::Option<super::core::RuntimeFractionalPercent>,
    #[prost(message, repeated, tag = "6")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(message, repeated, tag = "7")]
    pub query_parameters: ::prost::alloc::vec::Vec<QueryParameterMatcher>,
    #[prost(message, optional, tag = "8")]
    pub grpc: ::core::option::Option<route_match::GrpcRouteMatchOptions>,
    #[prost(message, optional, tag = "11")]
    pub tls_context: ::core::option::Option<route_match::TlsContextMatchOptions>,
    #[prost(oneof = "route_match::PathSpecifier", tags = "1, 2, 3, 10")]
    pub path_specifier: ::core::option::Option<route_match::PathSpecifier>,
}
/// Nested message and enum types in `RouteMatch`.
pub mod route_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GrpcRouteMatchOptions {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct TlsContextMatchOptions {
        #[prost(message, optional, tag = "1")]
        pub presented: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
        #[prost(message, optional, tag = "2")]
        pub validated: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathSpecifier {
        #[prost(string, tag = "1")]
        Prefix(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Path(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Regex(::prost::alloc::string::String),
        #[prost(message, tag = "10")]
        SafeRegex(super::super::super::super::r#type::matcher::RegexMatcher),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsPolicy {
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub allow_origin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(string, repeated, tag = "8")]
    pub allow_origin_regex: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "11")]
    pub allow_origin_string_match: ::prost::alloc::vec::Vec<
        super::super::super::r#type::matcher::StringMatcher,
    >,
    #[prost(string, tag = "2")]
    pub allow_methods: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub allow_headers: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub expose_headers: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub max_age: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub allow_credentials: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "10")]
    pub shadow_enabled: ::core::option::Option<super::core::RuntimeFractionalPercent>,
    #[prost(oneof = "cors_policy::EnabledSpecifier", tags = "7, 9")]
    pub enabled_specifier: ::core::option::Option<cors_policy::EnabledSpecifier>,
}
/// Nested message and enum types in `CorsPolicy`.
pub mod cors_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnabledSpecifier {
        #[prost(message, tag = "7")]
        Enabled(super::super::super::super::super::google::protobuf::BoolValue),
        #[prost(message, tag = "9")]
        FilterEnabled(super::super::core::RuntimeFractionalPercent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    #[prost(enumeration = "route_action::ClusterNotFoundResponseCode", tag = "20")]
    pub cluster_not_found_response_code: i32,
    #[prost(message, optional, tag = "4")]
    pub metadata_match: ::core::option::Option<super::core::Metadata>,
    #[prost(string, tag = "5")]
    pub prefix_rewrite: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "32")]
    pub regex_rewrite: ::core::option::Option<
        super::super::super::r#type::matcher::RegexMatchAndSubstitute,
    >,
    #[prost(message, optional, tag = "8")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "24")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "9")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    #[prost(message, optional, tag = "33")]
    pub retry_policy_typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub request_mirror_policy: ::core::option::Option<route_action::RequestMirrorPolicy>,
    #[prost(message, repeated, tag = "30")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<
        route_action::RequestMirrorPolicy,
    >,
    #[prost(enumeration = "super::core::RoutingPriority", tag = "11")]
    pub priority: i32,
    #[prost(message, repeated, tag = "13")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    #[prost(message, optional, tag = "14")]
    pub include_vh_rate_limits: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, repeated, tag = "15")]
    pub hash_policy: ::prost::alloc::vec::Vec<route_action::HashPolicy>,
    #[prost(message, optional, tag = "17")]
    pub cors: ::core::option::Option<CorsPolicy>,
    #[prost(message, optional, tag = "23")]
    pub max_grpc_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "28")]
    pub grpc_timeout_offset: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, repeated, tag = "25")]
    pub upgrade_configs: ::prost::alloc::vec::Vec<route_action::UpgradeConfig>,
    #[prost(enumeration = "route_action::InternalRedirectAction", tag = "26")]
    pub internal_redirect_action: i32,
    #[prost(message, optional, tag = "31")]
    pub max_internal_redirects: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "27")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    #[prost(oneof = "route_action::ClusterSpecifier", tags = "1, 2, 3")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
    #[prost(oneof = "route_action::HostRewriteSpecifier", tags = "6, 7, 29")]
    pub host_rewrite_specifier: ::core::option::Option<
        route_action::HostRewriteSpecifier,
    >,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestMirrorPolicy {
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        #[deprecated]
        #[prost(string, tag = "2")]
        pub runtime_key: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub runtime_fraction: ::core::option::Option<
            super::super::core::RuntimeFractionalPercent,
        >,
        #[prost(message, optional, tag = "4")]
        pub trace_sampled: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashPolicy {
        #[prost(bool, tag = "4")]
        pub terminal: bool,
        #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2, 3, 5, 6")]
        pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
    }
    /// Nested message and enum types in `HashPolicy`.
    pub mod hash_policy {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Header {
            #[prost(string, tag = "1")]
            pub header_name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Cookie {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "2")]
            pub ttl: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::Duration,
            >,
            #[prost(string, tag = "3")]
            pub path: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct ConnectionProperties {
            #[prost(bool, tag = "1")]
            pub source_ip: bool,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QueryParameter {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FilterState {
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicySpecifier {
            #[prost(message, tag = "1")]
            Header(Header),
            #[prost(message, tag = "2")]
            Cookie(Cookie),
            #[prost(message, tag = "3")]
            ConnectionProperties(ConnectionProperties),
            #[prost(message, tag = "5")]
            QueryParameter(QueryParameter),
            #[prost(message, tag = "6")]
            FilterState(FilterState),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeConfig {
        #[prost(string, tag = "1")]
        pub upgrade_type: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub enabled: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
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
    pub enum ClusterNotFoundResponseCode {
        ServiceUnavailable = 0,
        NotFound = 1,
    }
    impl ClusterNotFoundResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClusterNotFoundResponseCode::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                ClusterNotFoundResponseCode::NotFound => "NOT_FOUND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SERVICE_UNAVAILABLE" => Some(Self::ServiceUnavailable),
                "NOT_FOUND" => Some(Self::NotFound),
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
    pub enum InternalRedirectAction {
        PassThroughInternalRedirect = 0,
        HandleInternalRedirect = 1,
    }
    impl InternalRedirectAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InternalRedirectAction::PassThroughInternalRedirect => {
                    "PASS_THROUGH_INTERNAL_REDIRECT"
                }
                InternalRedirectAction::HandleInternalRedirect => {
                    "HANDLE_INTERNAL_REDIRECT"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PASS_THROUGH_INTERNAL_REDIRECT" => {
                    Some(Self::PassThroughInternalRedirect)
                }
                "HANDLE_INTERNAL_REDIRECT" => Some(Self::HandleInternalRedirect),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        #[prost(string, tag = "1")]
        Cluster(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        ClusterHeader(::prost::alloc::string::String),
        #[prost(message, tag = "3")]
        WeightedClusters(super::WeightedCluster),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostRewriteSpecifier {
        #[prost(string, tag = "6")]
        HostRewrite(::prost::alloc::string::String),
        #[prost(message, tag = "7")]
        AutoHostRewrite(super::super::super::super::super::google::protobuf::BoolValue),
        #[prost(string, tag = "29")]
        AutoHostRewriteHeader(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    #[prost(string, tag = "1")]
    pub retry_on: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub num_retries: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub per_try_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "4")]
    pub retry_priority: ::core::option::Option<retry_policy::RetryPriority>,
    #[prost(message, repeated, tag = "5")]
    pub retry_host_predicate: ::prost::alloc::vec::Vec<retry_policy::RetryHostPredicate>,
    #[prost(int64, tag = "6")]
    pub host_selection_retry_max_attempts: i64,
    #[prost(uint32, repeated, tag = "7")]
    pub retriable_status_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "8")]
    pub retry_back_off: ::core::option::Option<retry_policy::RetryBackOff>,
    #[prost(message, repeated, tag = "9")]
    pub retriable_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(message, repeated, tag = "10")]
    pub retriable_request_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
}
/// Nested message and enum types in `RetryPolicy`.
pub mod retry_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPriority {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "retry_priority::ConfigType", tags = "2, 3")]
        pub config_type: ::core::option::Option<retry_priority::ConfigType>,
    }
    /// Nested message and enum types in `RetryPriority`.
    pub mod retry_priority {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "2")]
            Config(super::super::super::super::super::super::google::protobuf::Struct),
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryHostPredicate {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "retry_host_predicate::ConfigType", tags = "2, 3")]
        pub config_type: ::core::option::Option<retry_host_predicate::ConfigType>,
    }
    /// Nested message and enum types in `RetryHostPredicate`.
    pub mod retry_host_predicate {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "2")]
            Config(super::super::super::super::super::super::google::protobuf::Struct),
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RetryBackOff {
        #[prost(message, optional, tag = "1")]
        pub base_interval: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        #[prost(message, optional, tag = "2")]
        pub max_interval: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HedgePolicy {
    #[prost(message, optional, tag = "1")]
    pub initial_requests: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "2")]
    pub additional_request_chance: ::core::option::Option<
        super::super::super::r#type::FractionalPercent,
    >,
    #[prost(bool, tag = "3")]
    pub hedge_on_per_try_timeout: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectAction {
    #[prost(string, tag = "1")]
    pub host_redirect: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub port_redirect: u32,
    #[prost(enumeration = "redirect_action::RedirectResponseCode", tag = "3")]
    pub response_code: i32,
    #[prost(bool, tag = "6")]
    pub strip_query: bool,
    #[prost(oneof = "redirect_action::SchemeRewriteSpecifier", tags = "4, 7")]
    pub scheme_rewrite_specifier: ::core::option::Option<
        redirect_action::SchemeRewriteSpecifier,
    >,
    #[prost(oneof = "redirect_action::PathRewriteSpecifier", tags = "2, 5")]
    pub path_rewrite_specifier: ::core::option::Option<
        redirect_action::PathRewriteSpecifier,
    >,
}
/// Nested message and enum types in `RedirectAction`.
pub mod redirect_action {
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
    pub enum RedirectResponseCode {
        MovedPermanently = 0,
        Found = 1,
        SeeOther = 2,
        TemporaryRedirect = 3,
        PermanentRedirect = 4,
    }
    impl RedirectResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RedirectResponseCode::MovedPermanently => "MOVED_PERMANENTLY",
                RedirectResponseCode::Found => "FOUND",
                RedirectResponseCode::SeeOther => "SEE_OTHER",
                RedirectResponseCode::TemporaryRedirect => "TEMPORARY_REDIRECT",
                RedirectResponseCode::PermanentRedirect => "PERMANENT_REDIRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOVED_PERMANENTLY" => Some(Self::MovedPermanently),
                "FOUND" => Some(Self::Found),
                "SEE_OTHER" => Some(Self::SeeOther),
                "TEMPORARY_REDIRECT" => Some(Self::TemporaryRedirect),
                "PERMANENT_REDIRECT" => Some(Self::PermanentRedirect),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SchemeRewriteSpecifier {
        #[prost(bool, tag = "4")]
        HttpsRedirect(bool),
        #[prost(string, tag = "7")]
        SchemeRedirect(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathRewriteSpecifier {
        #[prost(string, tag = "2")]
        PathRedirect(::prost::alloc::string::String),
        #[prost(string, tag = "5")]
        PrefixRewrite(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectResponseAction {
    #[prost(uint32, tag = "1")]
    pub status: u32,
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<super::core::DataSource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decorator {
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub propagate: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tracing {
    #[prost(message, optional, tag = "1")]
    pub client_sampling: ::core::option::Option<
        super::super::super::r#type::FractionalPercent,
    >,
    #[prost(message, optional, tag = "2")]
    pub random_sampling: ::core::option::Option<
        super::super::super::r#type::FractionalPercent,
    >,
    #[prost(message, optional, tag = "3")]
    pub overall_sampling: ::core::option::Option<
        super::super::super::r#type::FractionalPercent,
    >,
    #[prost(message, repeated, tag = "4")]
    pub custom_tags: ::prost::alloc::vec::Vec<
        super::super::super::r#type::tracing::v2::CustomTag,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualCluster {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub pattern: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(enumeration = "super::core::RequestMethod", tag = "3")]
    pub method: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    #[prost(message, optional, tag = "1")]
    pub stage: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(string, tag = "2")]
    pub disable_key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<rate_limit::Action>,
}
/// Nested message and enum types in `RateLimit`.
pub mod rate_limit {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(oneof = "action::ActionSpecifier", tags = "1, 2, 3, 4, 5, 6")]
        pub action_specifier: ::core::option::Option<action::ActionSpecifier>,
    }
    /// Nested message and enum types in `Action`.
    pub mod action {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct SourceCluster {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct DestinationCluster {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RequestHeaders {
            #[prost(string, tag = "1")]
            pub header_name: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub descriptor_key: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct RemoteAddress {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GenericKey {
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HeaderValueMatch {
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "2")]
            pub expect_match: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::BoolValue,
            >,
            #[prost(message, repeated, tag = "3")]
            pub headers: ::prost::alloc::vec::Vec<super::super::HeaderMatcher>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ActionSpecifier {
            #[prost(message, tag = "1")]
            SourceCluster(SourceCluster),
            #[prost(message, tag = "2")]
            DestinationCluster(DestinationCluster),
            #[prost(message, tag = "3")]
            RequestHeaders(RequestHeaders),
            #[prost(message, tag = "4")]
            RemoteAddress(RemoteAddress),
            #[prost(message, tag = "5")]
            GenericKey(GenericKey),
            #[prost(message, tag = "6")]
            HeaderValueMatch(HeaderValueMatch),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMatcher {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub invert_match: bool,
    #[prost(
        oneof = "header_matcher::HeaderMatchSpecifier",
        tags = "4, 5, 11, 6, 7, 9, 10"
    )]
    pub header_match_specifier: ::core::option::Option<
        header_matcher::HeaderMatchSpecifier,
    >,
}
/// Nested message and enum types in `HeaderMatcher`.
pub mod header_matcher {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HeaderMatchSpecifier {
        #[prost(string, tag = "4")]
        ExactMatch(::prost::alloc::string::String),
        #[prost(string, tag = "5")]
        RegexMatch(::prost::alloc::string::String),
        #[prost(message, tag = "11")]
        SafeRegexMatch(super::super::super::super::r#type::matcher::RegexMatcher),
        #[prost(message, tag = "6")]
        RangeMatch(super::super::super::super::r#type::Int64Range),
        #[prost(bool, tag = "7")]
        PresentMatch(bool),
        #[prost(string, tag = "9")]
        PrefixMatch(::prost::alloc::string::String),
        #[prost(string, tag = "10")]
        SuffixMatch(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameterMatcher {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(message, optional, tag = "4")]
    pub regex: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(
        oneof = "query_parameter_matcher::QueryParameterMatchSpecifier",
        tags = "5, 6"
    )]
    pub query_parameter_match_specifier: ::core::option::Option<
        query_parameter_matcher::QueryParameterMatchSpecifier,
    >,
}
/// Nested message and enum types in `QueryParameterMatcher`.
pub mod query_parameter_matcher {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryParameterMatchSpecifier {
        #[prost(message, tag = "5")]
        StringMatch(super::super::super::super::r#type::matcher::StringMatcher),
        #[prost(bool, tag = "6")]
        PresentMatch(bool),
    }
}
