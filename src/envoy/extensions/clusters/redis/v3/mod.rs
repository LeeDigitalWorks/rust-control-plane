// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod clusters {
            pub mod redis {
                pub mod v3 {
                    include!("envoy.extensions.clusters.redis.v3.rs");
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
