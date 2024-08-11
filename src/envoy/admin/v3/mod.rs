// This file is @generated by prost-build.
pub mod envoy {
    pub mod admin {
        pub mod v3 {
            include!("envoy.admin.v3.rs");
        }
    }
    pub mod config {
        pub mod common {
            pub mod matcher {
                pub mod v3 {
                    include!("envoy.config.common.matcher.v3.rs");
                }
            }
        }
        pub mod core {
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
            }
        }
        pub mod route {
            pub mod v3 {
                include!("envoy.config.route.v3.rs");
            }
        }
        pub mod tap {
            pub mod v3 {
                include!("envoy.config.tap.v3.rs");
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
        pub mod tracing {
            pub mod v3 {
                include!("envoy.r#type.tracing.v3.rs");
            }
        }
        pub mod v3 {
            include!("envoy.r#type.v3.rs");
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
    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!("xds.r#type.matcher.v3.rs");
            }
        }
    }
}
