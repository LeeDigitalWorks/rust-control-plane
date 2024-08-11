// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLog {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub filter: ::core::option::Option<AccessLogFilter>,
    #[prost(oneof = "access_log::ConfigType", tags = "4")]
    pub config_type: ::core::option::Option<access_log::ConfigType>,
}
/// Nested message and enum types in `AccessLog`.
pub mod access_log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "4")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLogFilter {
    #[prost(
        oneof = "access_log_filter::FilterSpecifier",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13"
    )]
    pub filter_specifier: ::core::option::Option<access_log_filter::FilterSpecifier>,
}
/// Nested message and enum types in `AccessLogFilter`.
pub mod access_log_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FilterSpecifier {
        #[prost(message, tag = "1")]
        StatusCodeFilter(super::StatusCodeFilter),
        #[prost(message, tag = "2")]
        DurationFilter(super::DurationFilter),
        #[prost(message, tag = "3")]
        NotHealthCheckFilter(super::NotHealthCheckFilter),
        #[prost(message, tag = "4")]
        TraceableFilter(super::TraceableFilter),
        #[prost(message, tag = "5")]
        RuntimeFilter(super::RuntimeFilter),
        #[prost(message, tag = "6")]
        AndFilter(super::AndFilter),
        #[prost(message, tag = "7")]
        OrFilter(super::OrFilter),
        #[prost(message, tag = "8")]
        HeaderFilter(super::HeaderFilter),
        #[prost(message, tag = "9")]
        ResponseFlagFilter(super::ResponseFlagFilter),
        #[prost(message, tag = "10")]
        GrpcStatusFilter(super::GrpcStatusFilter),
        #[prost(message, tag = "11")]
        ExtensionFilter(super::ExtensionFilter),
        #[prost(message, tag = "12")]
        MetadataFilter(super::MetadataFilter),
        #[prost(message, tag = "13")]
        LogTypeFilter(super::LogTypeFilter),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparisonFilter {
    #[prost(enumeration = "comparison_filter::Op", tag = "1")]
    pub op: i32,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<super::super::core::v3::RuntimeUInt32>,
}
/// Nested message and enum types in `ComparisonFilter`.
pub mod comparison_filter {
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
    pub enum Op {
        Eq = 0,
        Ge = 1,
        Le = 2,
    }
    impl Op {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Op::Eq => "EQ",
                Op::Ge => "GE",
                Op::Le => "LE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EQ" => Some(Self::Eq),
                "GE" => Some(Self::Ge),
                "LE" => Some(Self::Le),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCodeFilter {
    #[prost(message, optional, tag = "1")]
    pub comparison: ::core::option::Option<ComparisonFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DurationFilter {
    #[prost(message, optional, tag = "1")]
    pub comparison: ::core::option::Option<ComparisonFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NotHealthCheckFilter {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TraceableFilter {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFilter {
    #[prost(string, tag = "1")]
    pub runtime_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub percent_sampled: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(bool, tag = "3")]
    pub use_independent_randomness: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndFilter {
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<AccessLogFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrFilter {
    #[prost(message, repeated, tag = "2")]
    pub filters: ::prost::alloc::vec::Vec<AccessLogFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderFilter {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::route::v3::HeaderMatcher>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlagFilter {
    #[prost(string, repeated, tag = "1")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcStatusFilter {
    #[prost(
        enumeration = "grpc_status_filter::Status",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub statuses: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag = "2")]
    pub exclude: bool,
}
/// Nested message and enum types in `GrpcStatusFilter`.
pub mod grpc_status_filter {
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
    pub enum Status {
        Ok = 0,
        Canceled = 1,
        Unknown = 2,
        InvalidArgument = 3,
        DeadlineExceeded = 4,
        NotFound = 5,
        AlreadyExists = 6,
        PermissionDenied = 7,
        ResourceExhausted = 8,
        FailedPrecondition = 9,
        Aborted = 10,
        OutOfRange = 11,
        Unimplemented = 12,
        Internal = 13,
        Unavailable = 14,
        DataLoss = 15,
        Unauthenticated = 16,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Ok => "OK",
                Status::Canceled => "CANCELED",
                Status::Unknown => "UNKNOWN",
                Status::InvalidArgument => "INVALID_ARGUMENT",
                Status::DeadlineExceeded => "DEADLINE_EXCEEDED",
                Status::NotFound => "NOT_FOUND",
                Status::AlreadyExists => "ALREADY_EXISTS",
                Status::PermissionDenied => "PERMISSION_DENIED",
                Status::ResourceExhausted => "RESOURCE_EXHAUSTED",
                Status::FailedPrecondition => "FAILED_PRECONDITION",
                Status::Aborted => "ABORTED",
                Status::OutOfRange => "OUT_OF_RANGE",
                Status::Unimplemented => "UNIMPLEMENTED",
                Status::Internal => "INTERNAL",
                Status::Unavailable => "UNAVAILABLE",
                Status::DataLoss => "DATA_LOSS",
                Status::Unauthenticated => "UNAUTHENTICATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "CANCELED" => Some(Self::Canceled),
                "UNKNOWN" => Some(Self::Unknown),
                "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "NOT_FOUND" => Some(Self::NotFound),
                "ALREADY_EXISTS" => Some(Self::AlreadyExists),
                "PERMISSION_DENIED" => Some(Self::PermissionDenied),
                "RESOURCE_EXHAUSTED" => Some(Self::ResourceExhausted),
                "FAILED_PRECONDITION" => Some(Self::FailedPrecondition),
                "ABORTED" => Some(Self::Aborted),
                "OUT_OF_RANGE" => Some(Self::OutOfRange),
                "UNIMPLEMENTED" => Some(Self::Unimplemented),
                "INTERNAL" => Some(Self::Internal),
                "UNAVAILABLE" => Some(Self::Unavailable),
                "DATA_LOSS" => Some(Self::DataLoss),
                "UNAUTHENTICATED" => Some(Self::Unauthenticated),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataFilter {
    #[prost(message, optional, tag = "1")]
    pub matcher: ::core::option::Option<
        super::super::super::r#type::matcher::v3::MetadataMatcher,
    >,
    #[prost(message, optional, tag = "2")]
    pub match_if_key_not_found: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogTypeFilter {
    #[prost(
        enumeration = "super::super::super::data::accesslog::v3::AccessLogType",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub types: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, tag = "2")]
    pub exclude: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFilter {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "extension_filter::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<extension_filter::ConfigType>,
}
/// Nested message and enum types in `ExtensionFilter`.
pub mod extension_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
