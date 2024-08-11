// This file is @generated by prost-build.
pub mod envoy {
    pub mod api {
        pub mod v2 {
            pub mod cluster {
                include!("envoy.api.v2.cluster.rs");
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
