// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPair {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Gauge {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counter {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "2")]
    pub exemplar: ::core::option::Option<Exemplar>,
    #[prost(message, optional, tag = "3")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Quantile {
    #[prost(double, optional, tag = "1")]
    pub quantile: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Summary {
    #[prost(uint64, optional, tag = "1")]
    pub sample_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "2")]
    pub sample_sum: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "3")]
    pub quantile: ::prost::alloc::vec::Vec<Quantile>,
    #[prost(message, optional, tag = "4")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Untyped {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Histogram {
    #[prost(uint64, optional, tag = "1")]
    pub sample_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "4")]
    pub sample_count_float: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub sample_sum: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "3")]
    pub bucket: ::prost::alloc::vec::Vec<Bucket>,
    #[prost(message, optional, tag = "15")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(sint32, optional, tag = "5")]
    pub schema: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "6")]
    pub zero_threshold: ::core::option::Option<f64>,
    #[prost(uint64, optional, tag = "7")]
    pub zero_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "8")]
    pub zero_count_float: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "9")]
    pub negative_span: ::prost::alloc::vec::Vec<BucketSpan>,
    #[prost(sint64, repeated, packed = "false", tag = "10")]
    pub negative_delta: ::prost::alloc::vec::Vec<i64>,
    #[prost(double, repeated, packed = "false", tag = "11")]
    pub negative_count: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, repeated, tag = "12")]
    pub positive_span: ::prost::alloc::vec::Vec<BucketSpan>,
    #[prost(sint64, repeated, packed = "false", tag = "13")]
    pub positive_delta: ::prost::alloc::vec::Vec<i64>,
    #[prost(double, repeated, packed = "false", tag = "14")]
    pub positive_count: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, repeated, tag = "16")]
    pub exemplars: ::prost::alloc::vec::Vec<Exemplar>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(uint64, optional, tag = "1")]
    pub cumulative_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "4")]
    pub cumulative_count_float: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub upper_bound: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "3")]
    pub exemplar: ::core::option::Option<Exemplar>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BucketSpan {
    #[prost(sint32, optional, tag = "1")]
    pub offset: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub length: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exemplar {
    #[prost(message, repeated, tag = "1")]
    pub label: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(double, optional, tag = "2")]
    pub value: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(message, repeated, tag = "1")]
    pub label: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(message, optional, tag = "2")]
    pub gauge: ::core::option::Option<Gauge>,
    #[prost(message, optional, tag = "3")]
    pub counter: ::core::option::Option<Counter>,
    #[prost(message, optional, tag = "4")]
    pub summary: ::core::option::Option<Summary>,
    #[prost(message, optional, tag = "5")]
    pub untyped: ::core::option::Option<Untyped>,
    #[prost(message, optional, tag = "7")]
    pub histogram: ::core::option::Option<Histogram>,
    #[prost(int64, optional, tag = "6")]
    pub timestamp_ms: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricFamily {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub help: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "MetricType", optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub metric: ::prost::alloc::vec::Vec<Metric>,
    #[prost(string, optional, tag = "5")]
    pub unit: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    Counter = 0,
    Gauge = 1,
    Summary = 2,
    Untyped = 3,
    Histogram = 4,
    GaugeHistogram = 5,
}
impl MetricType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MetricType::Counter => "COUNTER",
            MetricType::Gauge => "GAUGE",
            MetricType::Summary => "SUMMARY",
            MetricType::Untyped => "UNTYPED",
            MetricType::Histogram => "HISTOGRAM",
            MetricType::GaugeHistogram => "GAUGE_HISTOGRAM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COUNTER" => Some(Self::Counter),
            "GAUGE" => Some(Self::Gauge),
            "SUMMARY" => Some(Self::Summary),
            "UNTYPED" => Some(Self::Untyped),
            "HISTOGRAM" => Some(Self::Histogram),
            "GAUGE_HISTOGRAM" => Some(Self::GaugeHistogram),
            _ => None,
        }
    }
}
