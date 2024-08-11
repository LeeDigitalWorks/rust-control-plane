// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod filters {
            pub mod network {
                pub mod echo {
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.echo.v3.rs");
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
