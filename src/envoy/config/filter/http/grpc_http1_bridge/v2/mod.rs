// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod filter {
            pub mod http {
                pub mod grpc_http1_bridge {
                    pub mod v2 {
                        include!("envoy.config.filter.http.grpc_http1_bridge.v2.rs");
                    }
                }
            }
        }
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
pub mod udpa {
    pub mod annotations {
        include!("udpa.annotations.rs");
    }
}
