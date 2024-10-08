// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Percent {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FractionalPercent {
    #[prost(uint32, tag = "1")]
    pub numerator: u32,
    #[prost(enumeration = "fractional_percent::DenominatorType", tag = "2")]
    pub denominator: i32,
}
/// Nested message and enum types in `FractionalPercent`.
pub mod fractional_percent {
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
    pub enum DenominatorType {
        Hundred = 0,
        TenThousand = 1,
        Million = 2,
    }
    impl DenominatorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DenominatorType::Hundred => "HUNDRED",
                DenominatorType::TenThousand => "TEN_THOUSAND",
                DenominatorType::Million => "MILLION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HUNDRED" => Some(Self::Hundred),
                "TEN_THOUSAND" => Some(Self::TenThousand),
                "MILLION" => Some(Self::Million),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SemanticVersion {
    #[prost(uint32, tag = "1")]
    pub major_number: u32,
    #[prost(uint32, tag = "2")]
    pub minor_number: u32,
    #[prost(uint32, tag = "3")]
    pub patch: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodecClientType {
    Http1 = 0,
    Http2 = 1,
    Http3 = 2,
}
impl CodecClientType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CodecClientType::Http1 => "HTTP1",
            CodecClientType::Http2 => "HTTP2",
            CodecClientType::Http3 => "HTTP3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP1" => Some(Self::Http1),
            "HTTP2" => Some(Self::Http2),
            "HTTP3" => Some(Self::Http3),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Int64Range {
    #[prost(int64, tag = "1")]
    pub start: i64,
    #[prost(int64, tag = "2")]
    pub end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Int32Range {
    #[prost(int32, tag = "1")]
    pub start: i32,
    #[prost(int32, tag = "2")]
    pub end: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DoubleRange {
    #[prost(double, tag = "1")]
    pub start: f64,
    #[prost(double, tag = "2")]
    pub end: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RateLimitUnit {
    Unknown = 0,
    Second = 1,
    Minute = 2,
    Hour = 3,
    Day = 4,
    Month = 5,
    Year = 6,
}
impl RateLimitUnit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RateLimitUnit::Unknown => "UNKNOWN",
            RateLimitUnit::Second => "SECOND",
            RateLimitUnit::Minute => "MINUTE",
            RateLimitUnit::Hour => "HOUR",
            RateLimitUnit::Day => "DAY",
            RateLimitUnit::Month => "MONTH",
            RateLimitUnit::Year => "YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SECOND" => Some(Self::Second),
            "MINUTE" => Some(Self::Minute),
            "HOUR" => Some(Self::Hour),
            "DAY" => Some(Self::Day),
            "MONTH" => Some(Self::Month),
            "YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TokenBucket {
    #[prost(uint32, tag = "1")]
    pub max_tokens: u32,
    #[prost(message, optional, tag = "2")]
    pub tokens_per_fill: ::core::option::Option<
        super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "3")]
    pub fill_interval: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpStatus {
    #[prost(enumeration = "StatusCode", tag = "1")]
    pub code: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusCode {
    Empty = 0,
    Continue = 100,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}
impl StatusCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatusCode::Empty => "Empty",
            StatusCode::Continue => "Continue",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "NonAuthoritativeInformation",
            StatusCode::NoContent => "NoContent",
            StatusCode::ResetContent => "ResetContent",
            StatusCode::PartialContent => "PartialContent",
            StatusCode::MultiStatus => "MultiStatus",
            StatusCode::AlreadyReported => "AlreadyReported",
            StatusCode::ImUsed => "IMUsed",
            StatusCode::MultipleChoices => "MultipleChoices",
            StatusCode::MovedPermanently => "MovedPermanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "SeeOther",
            StatusCode::NotModified => "NotModified",
            StatusCode::UseProxy => "UseProxy",
            StatusCode::TemporaryRedirect => "TemporaryRedirect",
            StatusCode::PermanentRedirect => "PermanentRedirect",
            StatusCode::BadRequest => "BadRequest",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "PaymentRequired",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "NotFound",
            StatusCode::MethodNotAllowed => "MethodNotAllowed",
            StatusCode::NotAcceptable => "NotAcceptable",
            StatusCode::ProxyAuthenticationRequired => "ProxyAuthenticationRequired",
            StatusCode::RequestTimeout => "RequestTimeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "LengthRequired",
            StatusCode::PreconditionFailed => "PreconditionFailed",
            StatusCode::PayloadTooLarge => "PayloadTooLarge",
            StatusCode::UriTooLong => "URITooLong",
            StatusCode::UnsupportedMediaType => "UnsupportedMediaType",
            StatusCode::RangeNotSatisfiable => "RangeNotSatisfiable",
            StatusCode::ExpectationFailed => "ExpectationFailed",
            StatusCode::MisdirectedRequest => "MisdirectedRequest",
            StatusCode::UnprocessableEntity => "UnprocessableEntity",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "FailedDependency",
            StatusCode::UpgradeRequired => "UpgradeRequired",
            StatusCode::PreconditionRequired => "PreconditionRequired",
            StatusCode::TooManyRequests => "TooManyRequests",
            StatusCode::RequestHeaderFieldsTooLarge => "RequestHeaderFieldsTooLarge",
            StatusCode::InternalServerError => "InternalServerError",
            StatusCode::NotImplemented => "NotImplemented",
            StatusCode::BadGateway => "BadGateway",
            StatusCode::ServiceUnavailable => "ServiceUnavailable",
            StatusCode::GatewayTimeout => "GatewayTimeout",
            StatusCode::HttpVersionNotSupported => "HTTPVersionNotSupported",
            StatusCode::VariantAlsoNegotiates => "VariantAlsoNegotiates",
            StatusCode::InsufficientStorage => "InsufficientStorage",
            StatusCode::LoopDetected => "LoopDetected",
            StatusCode::NotExtended => "NotExtended",
            StatusCode::NetworkAuthenticationRequired => "NetworkAuthenticationRequired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Empty" => Some(Self::Empty),
            "Continue" => Some(Self::Continue),
            "OK" => Some(Self::Ok),
            "Created" => Some(Self::Created),
            "Accepted" => Some(Self::Accepted),
            "NonAuthoritativeInformation" => Some(Self::NonAuthoritativeInformation),
            "NoContent" => Some(Self::NoContent),
            "ResetContent" => Some(Self::ResetContent),
            "PartialContent" => Some(Self::PartialContent),
            "MultiStatus" => Some(Self::MultiStatus),
            "AlreadyReported" => Some(Self::AlreadyReported),
            "IMUsed" => Some(Self::ImUsed),
            "MultipleChoices" => Some(Self::MultipleChoices),
            "MovedPermanently" => Some(Self::MovedPermanently),
            "Found" => Some(Self::Found),
            "SeeOther" => Some(Self::SeeOther),
            "NotModified" => Some(Self::NotModified),
            "UseProxy" => Some(Self::UseProxy),
            "TemporaryRedirect" => Some(Self::TemporaryRedirect),
            "PermanentRedirect" => Some(Self::PermanentRedirect),
            "BadRequest" => Some(Self::BadRequest),
            "Unauthorized" => Some(Self::Unauthorized),
            "PaymentRequired" => Some(Self::PaymentRequired),
            "Forbidden" => Some(Self::Forbidden),
            "NotFound" => Some(Self::NotFound),
            "MethodNotAllowed" => Some(Self::MethodNotAllowed),
            "NotAcceptable" => Some(Self::NotAcceptable),
            "ProxyAuthenticationRequired" => Some(Self::ProxyAuthenticationRequired),
            "RequestTimeout" => Some(Self::RequestTimeout),
            "Conflict" => Some(Self::Conflict),
            "Gone" => Some(Self::Gone),
            "LengthRequired" => Some(Self::LengthRequired),
            "PreconditionFailed" => Some(Self::PreconditionFailed),
            "PayloadTooLarge" => Some(Self::PayloadTooLarge),
            "URITooLong" => Some(Self::UriTooLong),
            "UnsupportedMediaType" => Some(Self::UnsupportedMediaType),
            "RangeNotSatisfiable" => Some(Self::RangeNotSatisfiable),
            "ExpectationFailed" => Some(Self::ExpectationFailed),
            "MisdirectedRequest" => Some(Self::MisdirectedRequest),
            "UnprocessableEntity" => Some(Self::UnprocessableEntity),
            "Locked" => Some(Self::Locked),
            "FailedDependency" => Some(Self::FailedDependency),
            "UpgradeRequired" => Some(Self::UpgradeRequired),
            "PreconditionRequired" => Some(Self::PreconditionRequired),
            "TooManyRequests" => Some(Self::TooManyRequests),
            "RequestHeaderFieldsTooLarge" => Some(Self::RequestHeaderFieldsTooLarge),
            "InternalServerError" => Some(Self::InternalServerError),
            "NotImplemented" => Some(Self::NotImplemented),
            "BadGateway" => Some(Self::BadGateway),
            "ServiceUnavailable" => Some(Self::ServiceUnavailable),
            "GatewayTimeout" => Some(Self::GatewayTimeout),
            "HTTPVersionNotSupported" => Some(Self::HttpVersionNotSupported),
            "VariantAlsoNegotiates" => Some(Self::VariantAlsoNegotiates),
            "InsufficientStorage" => Some(Self::InsufficientStorage),
            "LoopDetected" => Some(Self::LoopDetected),
            "NotExtended" => Some(Self::NotExtended),
            "NetworkAuthenticationRequired" => Some(Self::NetworkAuthenticationRequired),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RateLimitStrategy {
    #[prost(oneof = "rate_limit_strategy::Strategy", tags = "1, 2, 3")]
    pub strategy: ::core::option::Option<rate_limit_strategy::Strategy>,
}
/// Nested message and enum types in `RateLimitStrategy`.
pub mod rate_limit_strategy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RequestsPerTimeUnit {
        #[prost(uint64, tag = "1")]
        pub requests_per_time_unit: u64,
        #[prost(enumeration = "super::RateLimitUnit", tag = "2")]
        pub time_unit: i32,
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
    pub enum BlanketRule {
        AllowAll = 0,
        DenyAll = 1,
    }
    impl BlanketRule {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BlanketRule::AllowAll => "ALLOW_ALL",
                BlanketRule::DenyAll => "DENY_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALLOW_ALL" => Some(Self::AllowAll),
                "DENY_ALL" => Some(Self::DenyAll),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        #[prost(enumeration = "BlanketRule", tag = "1")]
        BlanketRule(i32),
        #[prost(message, tag = "2")]
        RequestsPerTimeUnit(RequestsPerTimeUnit),
        #[prost(message, tag = "3")]
        TokenBucket(super::TokenBucket),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashPolicy {
    #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2")]
    pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
}
/// Nested message and enum types in `HashPolicy`.
pub mod hash_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SourceIp {}
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
        SourceIp(SourceIp),
        #[prost(message, tag = "2")]
        FilterState(FilterState),
    }
}
