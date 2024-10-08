// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Body {
    #[prost(bool, tag = "3")]
    pub truncated: bool,
    #[prost(oneof = "body::BodyType", tags = "1, 2")]
    pub body_type: ::core::option::Option<body::BodyType>,
}
/// Nested message and enum types in `Body`.
pub mod body {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BodyType {
        #[prost(bytes, tag = "1")]
        AsBytes(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "2")]
        AsString(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    #[prost(message, optional, tag = "1")]
    pub local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    #[prost(message, optional, tag = "2")]
    pub remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpBufferedTrace {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<http_buffered_trace::Message>,
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<http_buffered_trace::Message>,
    #[prost(message, optional, tag = "3")]
    pub downstream_connection: ::core::option::Option<Connection>,
}
/// Nested message and enum types in `HttpBufferedTrace`.
pub mod http_buffered_trace {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        #[prost(message, repeated, tag = "1")]
        pub headers: ::prost::alloc::vec::Vec<
            super::super::super::super::config::core::v3::HeaderValue,
        >,
        #[prost(message, optional, tag = "2")]
        pub body: ::core::option::Option<super::Body>,
        #[prost(message, repeated, tag = "3")]
        pub trailers: ::prost::alloc::vec::Vec<
            super::super::super::super::config::core::v3::HeaderValue,
        >,
        #[prost(message, optional, tag = "4")]
        pub headers_received_time: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Timestamp,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpStreamedTraceSegment {
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    #[prost(
        oneof = "http_streamed_trace_segment::MessagePiece",
        tags = "2, 3, 4, 5, 6, 7"
    )]
    pub message_piece: ::core::option::Option<http_streamed_trace_segment::MessagePiece>,
}
/// Nested message and enum types in `HttpStreamedTraceSegment`.
pub mod http_streamed_trace_segment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessagePiece {
        #[prost(message, tag = "2")]
        RequestHeaders(super::super::super::super::config::core::v3::HeaderMap),
        #[prost(message, tag = "3")]
        RequestBodyChunk(super::Body),
        #[prost(message, tag = "4")]
        RequestTrailers(super::super::super::super::config::core::v3::HeaderMap),
        #[prost(message, tag = "5")]
        ResponseHeaders(super::super::super::super::config::core::v3::HeaderMap),
        #[prost(message, tag = "6")]
        ResponseBodyChunk(super::Body),
        #[prost(message, tag = "7")]
        ResponseTrailers(super::super::super::super::config::core::v3::HeaderMap),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketEvent {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(oneof = "socket_event::EventSelector", tags = "2, 3, 4")]
    pub event_selector: ::core::option::Option<socket_event::EventSelector>,
}
/// Nested message and enum types in `SocketEvent`.
pub mod socket_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Read {
        #[prost(message, optional, tag = "1")]
        pub data: ::core::option::Option<super::Body>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Write {
        #[prost(message, optional, tag = "1")]
        pub data: ::core::option::Option<super::Body>,
        #[prost(bool, tag = "2")]
        pub end_stream: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Closed {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventSelector {
        #[prost(message, tag = "2")]
        Read(Read),
        #[prost(message, tag = "3")]
        Write(Write),
        #[prost(message, tag = "4")]
        Closed(Closed),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketBufferedTrace {
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<SocketEvent>,
    #[prost(bool, tag = "4")]
    pub read_truncated: bool,
    #[prost(bool, tag = "5")]
    pub write_truncated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketStreamedTraceSegment {
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    #[prost(oneof = "socket_streamed_trace_segment::MessagePiece", tags = "2, 3")]
    pub message_piece: ::core::option::Option<
        socket_streamed_trace_segment::MessagePiece,
    >,
}
/// Nested message and enum types in `SocketStreamedTraceSegment`.
pub mod socket_streamed_trace_segment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessagePiece {
        #[prost(message, tag = "2")]
        Connection(super::Connection),
        #[prost(message, tag = "3")]
        Event(super::SocketEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceWrapper {
    #[prost(oneof = "trace_wrapper::Trace", tags = "1, 2, 3, 4")]
    pub trace: ::core::option::Option<trace_wrapper::Trace>,
}
/// Nested message and enum types in `TraceWrapper`.
pub mod trace_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trace {
        #[prost(message, tag = "1")]
        HttpBufferedTrace(super::HttpBufferedTrace),
        #[prost(message, tag = "2")]
        HttpStreamedTraceSegment(super::HttpStreamedTraceSegment),
        #[prost(message, tag = "3")]
        SocketBufferedTrace(super::SocketBufferedTrace),
        #[prost(message, tag = "4")]
        SocketStreamedTraceSegment(super::SocketStreamedTraceSegment),
    }
}
