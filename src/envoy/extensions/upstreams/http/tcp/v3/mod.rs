// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod upstreams {
            pub mod http {
                pub mod tcp {
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.tcp.v3.rs");
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
