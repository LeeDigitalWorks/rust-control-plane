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
        pub mod wasm {
            pub mod v3 {
                include!("envoy.extensions.wasm.v3.rs");
            }
        }
    }
    pub mod r#type {
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
