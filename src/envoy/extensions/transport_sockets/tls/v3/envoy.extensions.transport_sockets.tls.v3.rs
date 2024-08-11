// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpiffeCertValidatorConfig {
    #[prost(message, repeated, tag = "1")]
    pub trust_domains: ::prost::alloc::vec::Vec<
        spiffe_cert_validator_config::TrustDomain,
    >,
}
/// Nested message and enum types in `SPIFFECertValidatorConfig`.
pub mod spiffe_cert_validator_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrustDomain {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub trust_bundle: ::core::option::Option<
            super::super::super::super::super::config::core::v3::DataSource,
        >,
    }
}
