// This file is @generated by prost-build.
pub mod envoy {
    pub mod api {
        pub mod v2 {
            pub mod core {
                include!("envoy.api.v2.core.rs");
            }
            pub mod ratelimit {
                include!("envoy.api.v2.ratelimit.rs");
            }
        }
    }
    pub mod config {
        pub mod filter {
            pub mod network {
                pub mod rate_limit {
                    pub mod v2 {
                        include!("envoy.config.filter.network.rate_limit.v2.rs");
                    }
                }
            }
        }
        pub mod ratelimit {
            pub mod v2 {
                include!("envoy.config.ratelimit.v2.rs");
            }
        }
    }
    pub mod r#type {
        include!("envoy.r#type.rs");
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
