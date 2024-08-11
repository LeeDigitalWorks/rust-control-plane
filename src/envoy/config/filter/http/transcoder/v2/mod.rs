// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod filter {
            pub mod http {
                pub mod transcoder {
                    pub mod v2 {
                        include!("envoy.config.filter.http.transcoder.v2.rs");
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
pub mod validate {
    include!("validate.rs");
}
