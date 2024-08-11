// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod cluster {
            pub mod v3 {
                include!("envoy.config.cluster.v3.rs");
            }
        }
        pub mod common {
            pub mod key_value {
                pub mod v3 {
                    include!("envoy.config.common.key_value.v3.rs");
                }
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
    pub mod extensions {
        pub mod clusters {
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!("envoy.extensions.clusters.dynamic_forward_proxy.v3.rs");
                }
            }
        }
        pub mod common {
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!("envoy.extensions.common.dynamic_forward_proxy.v3.rs");
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
}
