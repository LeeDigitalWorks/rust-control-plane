// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FluentdAccessLogConfig {
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub buffer_flush_interval: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "5")]
    pub buffer_size_bytes: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    #[prost(message, optional, tag = "6")]
    pub record: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Struct,
    >,
    #[prost(message, optional, tag = "7")]
    pub retry_options: ::core::option::Option<fluentd_access_log_config::RetryOptions>,
    #[prost(message, repeated, tag = "8")]
    pub formatters: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
/// Nested message and enum types in `FluentdAccessLogConfig`.
pub mod fluentd_access_log_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RetryOptions {
        #[prost(message, optional, tag = "1")]
        pub max_connect_attempts: ::core::option::Option<
            super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        #[prost(message, optional, tag = "2")]
        pub backoff_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::BackoffStrategy,
        >,
    }
}
