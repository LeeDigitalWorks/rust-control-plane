// This file is @generated by prost-build.
pub mod envoy {
    pub mod api {
        pub mod v2 {
            pub mod core {
                include!("envoy.api.v2.core.rs");
            }
        }
    }
    pub mod config {
        pub mod filter {
            pub mod http {
                pub mod ip_tagging {
                    pub mod v2 {
                        include!("envoy.config.filter.http.ip_tagging.v2.rs");
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
