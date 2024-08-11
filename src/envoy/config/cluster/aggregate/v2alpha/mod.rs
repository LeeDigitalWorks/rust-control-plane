// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod cluster {
            pub mod aggregate {
                pub mod v2alpha {
                    include!("envoy.config.cluster.aggregate.v2alpha.rs");
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
