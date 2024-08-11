// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod filters {
            pub mod network {
                pub mod generic_proxy {
                    pub mod router {
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.generic_proxy.router.v3.rs");
                        }
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
pub mod xds {
    pub mod annotations {
        pub mod v3 {
            include!("xds.annotations.v3.rs");
        }
    }
}
