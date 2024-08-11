// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMonitor {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "resource_monitor::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<resource_monitor::ConfigType>,
}
/// Nested message and enum types in `ResourceMonitor`.
pub mod resource_monitor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ThresholdTrigger {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ScaledTrigger {
    #[prost(double, tag = "1")]
    pub scaling_threshold: f64,
    #[prost(double, tag = "2")]
    pub saturation_threshold: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "trigger::TriggerOneof", tags = "2, 3")]
    pub trigger_oneof: ::core::option::Option<trigger::TriggerOneof>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum TriggerOneof {
        #[prost(message, tag = "2")]
        Threshold(super::ThresholdTrigger),
        #[prost(message, tag = "3")]
        Scaled(super::ScaledTrigger),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleTimersOverloadActionConfig {
    #[prost(message, repeated, tag = "1")]
    pub timer_scale_factors: ::prost::alloc::vec::Vec<
        scale_timers_overload_action_config::ScaleTimer,
    >,
}
/// Nested message and enum types in `ScaleTimersOverloadActionConfig`.
pub mod scale_timers_overload_action_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ScaleTimer {
        #[prost(enumeration = "TimerType", tag = "1")]
        pub timer: i32,
        #[prost(oneof = "scale_timer::OverloadAdjust", tags = "2, 3")]
        pub overload_adjust: ::core::option::Option<scale_timer::OverloadAdjust>,
    }
    /// Nested message and enum types in `ScaleTimer`.
    pub mod scale_timer {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
        pub enum OverloadAdjust {
            #[prost(message, tag = "2")]
            MinTimeout(
                super::super::super::super::super::super::google::protobuf::Duration,
            ),
            #[prost(message, tag = "3")]
            MinScale(super::super::super::super::super::r#type::v3::Percent),
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
    pub enum TimerType {
        Unspecified = 0,
        HttpDownstreamConnectionIdle = 1,
        HttpDownstreamStreamIdle = 2,
        TransportSocketConnect = 3,
    }
    impl TimerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimerType::Unspecified => "UNSPECIFIED",
                TimerType::HttpDownstreamConnectionIdle => {
                    "HTTP_DOWNSTREAM_CONNECTION_IDLE"
                }
                TimerType::HttpDownstreamStreamIdle => "HTTP_DOWNSTREAM_STREAM_IDLE",
                TimerType::TransportSocketConnect => "TRANSPORT_SOCKET_CONNECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "HTTP_DOWNSTREAM_CONNECTION_IDLE" => {
                    Some(Self::HttpDownstreamConnectionIdle)
                }
                "HTTP_DOWNSTREAM_STREAM_IDLE" => Some(Self::HttpDownstreamStreamIdle),
                "TRANSPORT_SOCKET_CONNECT" => Some(Self::TransportSocketConnect),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadAction {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    #[prost(message, optional, tag = "3")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadShedPoint {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BufferFactoryConfig {
    #[prost(uint32, tag = "1")]
    pub minimum_account_to_track_power_of_two: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadManager {
    #[prost(message, optional, tag = "1")]
    pub refresh_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, repeated, tag = "2")]
    pub resource_monitors: ::prost::alloc::vec::Vec<ResourceMonitor>,
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<OverloadAction>,
    #[prost(message, repeated, tag = "5")]
    pub loadshed_points: ::prost::alloc::vec::Vec<LoadShedPoint>,
    #[prost(message, optional, tag = "4")]
    pub buffer_factory_config: ::core::option::Option<BufferFactoryConfig>,
}
