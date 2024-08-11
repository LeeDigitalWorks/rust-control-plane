// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod core {
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
            }
        }
        pub mod rbac {
            pub mod v3 {
                include!("envoy.config.rbac.v3.rs");
            }
        }
        pub mod route {
            pub mod v3 {
                include!("envoy.config.route.v3.rs");
            }
        }
    }
    pub mod extensions {
        pub mod filters {
            pub mod http {
                pub mod rbac {
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.rbac.v3.rs");
                    }
                }
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
    pub mod api {
        pub mod expr {
            pub mod v1alpha1 {
                include!("google.api.expr.v1alpha1.rs");
            }
        }
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
    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!("xds.r#type.matcher.v3.rs");
            }
        }
    }
}
