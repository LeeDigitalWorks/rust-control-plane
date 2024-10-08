// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bootstrap {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<super::super::core::v3::Node>,
    #[prost(string, repeated, tag = "26")]
    pub node_context_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub static_resources: ::core::option::Option<bootstrap::StaticResources>,
    #[prost(message, optional, tag = "3")]
    pub dynamic_resources: ::core::option::Option<bootstrap::DynamicResources>,
    #[prost(message, optional, tag = "4")]
    pub cluster_manager: ::core::option::Option<ClusterManager>,
    #[prost(message, optional, tag = "14")]
    pub hds_config: ::core::option::Option<super::super::core::v3::ApiConfigSource>,
    #[prost(string, tag = "5")]
    pub flags_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub stats_sinks: ::prost::alloc::vec::Vec<super::super::metrics::v3::StatsSink>,
    #[prost(message, optional, tag = "39")]
    pub deferred_stat_options: ::core::option::Option<bootstrap::DeferredStatOptions>,
    #[prost(message, optional, tag = "13")]
    pub stats_config: ::core::option::Option<super::super::metrics::v3::StatsConfig>,
    #[prost(message, optional, tag = "7")]
    pub stats_flush_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub watchdog: ::core::option::Option<Watchdog>,
    #[prost(message, optional, tag = "27")]
    pub watchdogs: ::core::option::Option<Watchdogs>,
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub tracing: ::core::option::Option<super::super::trace::v3::Tracing>,
    #[prost(message, optional, tag = "17")]
    pub layered_runtime: ::core::option::Option<LayeredRuntime>,
    #[prost(message, optional, tag = "12")]
    pub admin: ::core::option::Option<Admin>,
    #[prost(message, optional, tag = "15")]
    pub overload_manager: ::core::option::Option<
        super::super::overload::v3::OverloadManager,
    >,
    #[prost(bool, tag = "16")]
    pub enable_dispatcher_stats: bool,
    #[prost(string, tag = "18")]
    pub header_prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "19")]
    pub stats_server_version_override: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt64Value,
    >,
    #[deprecated]
    #[prost(bool, tag = "20")]
    pub use_tcp_for_dns_lookups: bool,
    #[deprecated]
    #[prost(message, optional, tag = "30")]
    pub dns_resolution_config: ::core::option::Option<
        super::super::core::v3::DnsResolutionConfig,
    >,
    #[prost(message, optional, tag = "31")]
    pub typed_dns_resolver_config: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, repeated, tag = "21")]
    pub bootstrap_extensions: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, repeated, tag = "28")]
    pub fatal_actions: ::prost::alloc::vec::Vec<FatalAction>,
    #[prost(message, repeated, tag = "22")]
    pub config_sources: ::prost::alloc::vec::Vec<super::super::core::v3::ConfigSource>,
    #[prost(message, optional, tag = "23")]
    pub default_config_source: ::core::option::Option<
        super::super::core::v3::ConfigSource,
    >,
    #[prost(string, tag = "24")]
    pub default_socket_interface: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "25")]
    pub certificate_provider_instances: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, repeated, tag = "32")]
    pub inline_headers: ::prost::alloc::vec::Vec<CustomInlineHeader>,
    #[prost(string, tag = "33")]
    pub perf_tracing_file_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "34")]
    pub default_regex_engine: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "35")]
    pub xds_delegate_extension: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "36")]
    pub xds_config_tracker_extension: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "37")]
    pub listener_manager: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    #[prost(message, optional, tag = "38")]
    pub application_log_config: ::core::option::Option<bootstrap::ApplicationLogConfig>,
    #[prost(message, optional, tag = "40")]
    pub grpc_async_client_manager_config: ::core::option::Option<
        bootstrap::GrpcAsyncClientManagerConfig,
    >,
    #[prost(message, optional, tag = "41")]
    pub memory_allocator_manager: ::core::option::Option<MemoryAllocatorManager>,
    #[prost(oneof = "bootstrap::StatsFlush", tags = "29")]
    pub stats_flush: ::core::option::Option<bootstrap::StatsFlush>,
}
/// Nested message and enum types in `Bootstrap`.
pub mod bootstrap {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticResources {
        #[prost(message, repeated, tag = "1")]
        pub listeners: ::prost::alloc::vec::Vec<
            super::super::super::listener::v3::Listener,
        >,
        #[prost(message, repeated, tag = "2")]
        pub clusters: ::prost::alloc::vec::Vec<
            super::super::super::cluster::v3::Cluster,
        >,
        #[prost(message, repeated, tag = "3")]
        pub secrets: ::prost::alloc::vec::Vec<
            super::super::super::super::extensions::transport_sockets::tls::v3::Secret,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicResources {
        #[prost(message, optional, tag = "1")]
        pub lds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
        #[prost(string, tag = "5")]
        pub lds_resources_locator: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub cds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
        #[prost(string, tag = "6")]
        pub cds_resources_locator: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub ads_config: ::core::option::Option<
            super::super::super::core::v3::ApiConfigSource,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationLogConfig {
        #[prost(message, optional, tag = "1")]
        pub log_format: ::core::option::Option<application_log_config::LogFormat>,
    }
    /// Nested message and enum types in `ApplicationLogConfig`.
    pub mod application_log_config {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LogFormat {
            #[prost(oneof = "log_format::LogFormat", tags = "1, 2")]
            pub log_format: ::core::option::Option<log_format::LogFormat>,
        }
        /// Nested message and enum types in `LogFormat`.
        pub mod log_format {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum LogFormat {
                #[prost(message, tag = "1")]
                JsonFormat(
                    super::super::super::super::super::super::super::google::protobuf::Struct,
                ),
                #[prost(string, tag = "2")]
                TextFormat(::prost::alloc::string::String),
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DeferredStatOptions {
        #[prost(bool, tag = "1")]
        pub enable_deferred_creation_stats: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GrpcAsyncClientManagerConfig {
        #[prost(message, optional, tag = "1")]
        pub max_cached_entry_idle_duration: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum StatsFlush {
        #[prost(bool, tag = "29")]
        StatsFlushOnAdmin(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Admin {
    #[prost(message, repeated, tag = "5")]
    pub access_log: ::prost::alloc::vec::Vec<super::super::accesslog::v3::AccessLog>,
    #[deprecated]
    #[prost(string, tag = "1")]
    pub access_log_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub profile_path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    #[prost(message, repeated, tag = "4")]
    pub socket_options: ::prost::alloc::vec::Vec<super::super::core::v3::SocketOption>,
    #[prost(bool, tag = "6")]
    pub ignore_global_conn_limit: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterManager {
    #[prost(string, tag = "1")]
    pub local_cluster_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub outlier_detection: ::core::option::Option<cluster_manager::OutlierDetection>,
    #[prost(message, optional, tag = "3")]
    pub upstream_bind_config: ::core::option::Option<super::super::core::v3::BindConfig>,
    #[prost(message, optional, tag = "4")]
    pub load_stats_config: ::core::option::Option<
        super::super::core::v3::ApiConfigSource,
    >,
    #[prost(bool, tag = "5")]
    pub enable_deferred_cluster_creation: bool,
}
/// Nested message and enum types in `ClusterManager`.
pub mod cluster_manager {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutlierDetection {
        #[prost(string, tag = "1")]
        pub event_log_path: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub event_service: ::core::option::Option<
            super::super::super::core::v3::EventServiceConfig,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watchdogs {
    #[prost(message, optional, tag = "1")]
    pub main_thread_watchdog: ::core::option::Option<Watchdog>,
    #[prost(message, optional, tag = "2")]
    pub worker_watchdog: ::core::option::Option<Watchdog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watchdog {
    #[prost(message, repeated, tag = "7")]
    pub actions: ::prost::alloc::vec::Vec<watchdog::WatchdogAction>,
    #[prost(message, optional, tag = "1")]
    pub miss_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "2")]
    pub megamiss_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "3")]
    pub kill_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "6")]
    pub max_kill_timeout_jitter: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "4")]
    pub multikill_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, optional, tag = "5")]
    pub multikill_threshold: ::core::option::Option<
        super::super::super::r#type::v3::Percent,
    >,
}
/// Nested message and enum types in `Watchdog`.
pub mod watchdog {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WatchdogAction {
        #[prost(message, optional, tag = "1")]
        pub config: ::core::option::Option<
            super::super::super::core::v3::TypedExtensionConfig,
        >,
        #[prost(enumeration = "watchdog_action::WatchdogEvent", tag = "2")]
        pub event: i32,
    }
    /// Nested message and enum types in `WatchdogAction`.
    pub mod watchdog_action {
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
        pub enum WatchdogEvent {
            Unknown = 0,
            Kill = 1,
            Multikill = 2,
            Megamiss = 3,
            Miss = 4,
        }
        impl WatchdogEvent {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    WatchdogEvent::Unknown => "UNKNOWN",
                    WatchdogEvent::Kill => "KILL",
                    WatchdogEvent::Multikill => "MULTIKILL",
                    WatchdogEvent::Megamiss => "MEGAMISS",
                    WatchdogEvent::Miss => "MISS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "KILL" => Some(Self::Kill),
                    "MULTIKILL" => Some(Self::Multikill),
                    "MEGAMISS" => Some(Self::Megamiss),
                    "MISS" => Some(Self::Miss),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FatalAction {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    #[prost(string, tag = "1")]
    pub symlink_root: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subdirectory: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub override_subdirectory: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub base: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeLayer {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "runtime_layer::LayerSpecifier", tags = "2, 3, 4, 5")]
    pub layer_specifier: ::core::option::Option<runtime_layer::LayerSpecifier>,
}
/// Nested message and enum types in `RuntimeLayer`.
pub mod runtime_layer {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiskLayer {
        #[prost(string, tag = "1")]
        pub symlink_root: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub subdirectory: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub append_service_cluster: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct AdminLayer {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RtdsLayer {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub rtds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LayerSpecifier {
        #[prost(message, tag = "2")]
        StaticLayer(super::super::super::super::super::google::protobuf::Struct),
        #[prost(message, tag = "3")]
        DiskLayer(DiskLayer),
        #[prost(message, tag = "4")]
        AdminLayer(AdminLayer),
        #[prost(message, tag = "5")]
        RtdsLayer(RtdsLayer),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayeredRuntime {
    #[prost(message, repeated, tag = "1")]
    pub layers: ::prost::alloc::vec::Vec<RuntimeLayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInlineHeader {
    #[prost(string, tag = "1")]
    pub inline_header_name: ::prost::alloc::string::String,
    #[prost(enumeration = "custom_inline_header::InlineHeaderType", tag = "2")]
    pub inline_header_type: i32,
}
/// Nested message and enum types in `CustomInlineHeader`.
pub mod custom_inline_header {
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
    pub enum InlineHeaderType {
        RequestHeader = 0,
        RequestTrailer = 1,
        ResponseHeader = 2,
        ResponseTrailer = 3,
    }
    impl InlineHeaderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InlineHeaderType::RequestHeader => "REQUEST_HEADER",
                InlineHeaderType::RequestTrailer => "REQUEST_TRAILER",
                InlineHeaderType::ResponseHeader => "RESPONSE_HEADER",
                InlineHeaderType::ResponseTrailer => "RESPONSE_TRAILER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUEST_HEADER" => Some(Self::RequestHeader),
                "REQUEST_TRAILER" => Some(Self::RequestTrailer),
                "RESPONSE_HEADER" => Some(Self::ResponseHeader),
                "RESPONSE_TRAILER" => Some(Self::ResponseTrailer),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MemoryAllocatorManager {
    #[prost(uint64, tag = "1")]
    pub bytes_to_release: u64,
    #[prost(message, optional, tag = "2")]
    pub memory_release_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
