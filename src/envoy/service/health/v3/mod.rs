// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod cluster {
            pub mod v3 {
                include!("envoy.config.cluster.v3.rs");
            }
        }
        pub mod core {
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
            }
        }
        pub mod endpoint {
            pub mod v3 {
                include!("envoy.config.endpoint.v3.rs");
            }
        }
    }
    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!("envoy.r#type.matcher.v3.rs");
            }
        }
        pub mod metadata {
            pub mod v3 {
                include!("envoy.r#type.metadata.v3.rs");
            }
        }
        pub mod v3 {
            include!("envoy.r#type.v3.rs");
        }
    }
    pub mod service {
        pub mod health {
            pub mod v3 {
                include!("envoy.service.health.v3.rs");
            }
        }
    }
}
pub mod google {
    pub mod api {
        include!("google.api.rs");
    }
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
pub mod xds {
    pub mod annotations {
        pub mod v3 {
            include!("xds.annotations.v3.rs");
        }
    }
    pub mod core {
        pub mod v3 {
            include!("xds.core.v3.rs");
        }
    }
}
