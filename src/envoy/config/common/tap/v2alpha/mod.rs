// This file is @generated by prost-build.
pub mod envoy {
    pub mod api {
        pub mod v2 {
            pub mod core {
                include!("envoy.api.v2.core.rs");
            }
            pub mod route {
                include!("envoy.api.v2.route.rs");
            }
        }
    }
    pub mod config {
        pub mod common {
            pub mod tap {
                pub mod v2alpha {
                    include!("envoy.config.common.tap.v2alpha.rs");
                }
            }
        }
    }
    pub mod r#type {
        include!("envoy.r#type.rs");
        pub mod matcher {
            include!("envoy.r#type.matcher.rs");
        }
        pub mod metadata {
            pub mod v2 {
                include!("envoy.r#type.metadata.v2.rs");
            }
        }
        pub mod tracing {
            pub mod v2 {
                include!("envoy.r#type.tracing.v2.rs");
            }
        }
    }
    pub mod service {
        pub mod tap {
            pub mod v2alpha {
                include!("envoy.service.tap.v2alpha.rs");
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
