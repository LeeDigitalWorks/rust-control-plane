// This file is @generated by prost-build.
pub mod envoy {
    pub mod api {
        pub mod v2 {
            pub mod core {
                include!("envoy.api.v2.core.rs");
            }
        }
    }
    pub mod r#type {
        include!("envoy.r#type.rs");
    }
    pub mod service {
        pub mod metrics {
            pub mod v2 {
                include!("envoy.service.metrics.v2.rs");
            }
        }
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
pub mod io {
    pub mod prometheus {
        pub mod client {
            include!("io.prometheus.client.rs");
        }
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
