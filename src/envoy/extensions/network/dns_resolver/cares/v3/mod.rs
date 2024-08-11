// This file is @generated by prost-build.
pub mod envoy {
    pub mod config {
        pub mod core {
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
            }
        }
    }
    pub mod extensions {
        pub mod network {
            pub mod dns_resolver {
                pub mod cares {
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.cares.v3.rs");
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
pub mod validate {
    include!("validate.rs");
}
