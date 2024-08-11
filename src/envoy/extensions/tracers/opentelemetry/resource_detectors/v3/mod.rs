// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod tracers {
            pub mod opentelemetry {
                pub mod resource_detectors {
                    pub mod v3 {
                        include!("envoy.extensions.tracers.opentelemetry.resource_detectors.v3.rs");
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
