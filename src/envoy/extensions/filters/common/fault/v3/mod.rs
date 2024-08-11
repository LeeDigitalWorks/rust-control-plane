// This file is @generated by prost-build.
pub mod envoy {
    pub mod extensions {
        pub mod filters {
            pub mod common {
                pub mod fault {
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.fault.v3.rs");
                    }
                }
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
