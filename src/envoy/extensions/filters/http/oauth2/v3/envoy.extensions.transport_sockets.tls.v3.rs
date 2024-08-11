// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsParameters {
    #[prost(enumeration = "tls_parameters::TlsProtocol", tag = "1")]
    pub tls_minimum_protocol_version: i32,
    #[prost(enumeration = "tls_parameters::TlsProtocol", tag = "2")]
    pub tls_maximum_protocol_version: i32,
    #[prost(string, repeated, tag = "3")]
    pub cipher_suites: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub ecdh_curves: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub signature_algorithms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `TlsParameters`.
pub mod tls_parameters {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TlsProtocol {
        TlsAuto = 0,
        TlSv10 = 1,
        TlSv11 = 2,
        TlSv12 = 3,
        TlSv13 = 4,
    }
    impl TlsProtocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsProtocol::TlsAuto => "TLS_AUTO",
                TlsProtocol::TlSv10 => "TLSv1_0",
                TlsProtocol::TlSv11 => "TLSv1_1",
                TlsProtocol::TlSv12 => "TLSv1_2",
                TlsProtocol::TlSv13 => "TLSv1_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TLS_AUTO" => Some(Self::TlsAuto),
                "TLSv1_0" => Some(Self::TlSv10),
                "TLSv1_1" => Some(Self::TlSv11),
                "TLSv1_2" => Some(Self::TlSv12),
                "TLSv1_3" => Some(Self::TlSv13),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyProvider {
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub fallback: bool,
    #[prost(oneof = "private_key_provider::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<private_key_provider::ConfigType>,
}
/// Nested message and enum types in `PrivateKeyProvider`.
pub mod private_key_provider {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsCertificate {
    #[prost(message, optional, tag = "1")]
    pub certificate_chain: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "2")]
    pub private_key: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "8")]
    pub pkcs12: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "7")]
    pub watched_directory: ::core::option::Option<
        super::super::super::super::config::core::v3::WatchedDirectory,
    >,
    #[prost(message, optional, tag = "6")]
    pub private_key_provider: ::core::option::Option<PrivateKeyProvider>,
    #[prost(message, optional, tag = "3")]
    pub password: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "4")]
    pub ocsp_staple: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, repeated, tag = "5")]
    pub signed_certificate_timestamp: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsSessionTicketKeys {
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateProviderPluginInstance {
    #[prost(string, tag = "1")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub certificate_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAltNameMatcher {
    #[prost(enumeration = "subject_alt_name_matcher::SanType", tag = "1")]
    pub san_type: i32,
    #[prost(message, optional, tag = "2")]
    pub matcher: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    #[prost(string, tag = "3")]
    pub oid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SubjectAltNameMatcher`.
pub mod subject_alt_name_matcher {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SanType {
        Unspecified = 0,
        Email = 1,
        Dns = 2,
        Uri = 3,
        IpAddress = 4,
        OtherName = 5,
    }
    impl SanType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SanType::Unspecified => "SAN_TYPE_UNSPECIFIED",
                SanType::Email => "EMAIL",
                SanType::Dns => "DNS",
                SanType::Uri => "URI",
                SanType::IpAddress => "IP_ADDRESS",
                SanType::OtherName => "OTHER_NAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SAN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EMAIL" => Some(Self::Email),
                "DNS" => Some(Self::Dns),
                "URI" => Some(Self::Uri),
                "IP_ADDRESS" => Some(Self::IpAddress),
                "OTHER_NAME" => Some(Self::OtherName),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateValidationContext {
    #[prost(message, optional, tag = "1")]
    pub trusted_ca: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(message, optional, tag = "13")]
    pub ca_certificate_provider_instance: ::core::option::Option<
        CertificateProviderPluginInstance,
    >,
    #[prost(message, optional, tag = "17")]
    pub system_root_certs: ::core::option::Option<
        certificate_validation_context::SystemRootCerts,
    >,
    #[prost(message, optional, tag = "11")]
    pub watched_directory: ::core::option::Option<
        super::super::super::super::config::core::v3::WatchedDirectory,
    >,
    #[prost(string, repeated, tag = "3")]
    pub verify_certificate_spki: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "2")]
    pub verify_certificate_hash: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "15")]
    pub match_typed_subject_alt_names: ::prost::alloc::vec::Vec<SubjectAltNameMatcher>,
    #[deprecated]
    #[prost(message, repeated, tag = "9")]
    pub match_subject_alt_names: ::prost::alloc::vec::Vec<
        super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    #[prost(message, optional, tag = "6")]
    pub require_signed_certificate_timestamp: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "7")]
    pub crl: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    #[prost(bool, tag = "8")]
    pub allow_expired_certificate: bool,
    #[prost(
        enumeration = "certificate_validation_context::TrustChainVerification",
        tag = "10"
    )]
    pub trust_chain_verification: i32,
    #[prost(message, optional, tag = "12")]
    pub custom_validator_config: ::core::option::Option<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    #[prost(bool, tag = "14")]
    pub only_verify_leaf_cert_crl: bool,
    #[prost(message, optional, tag = "16")]
    pub max_verify_depth: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `CertificateValidationContext`.
pub mod certificate_validation_context {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SystemRootCerts {}
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TrustChainVerification {
        VerifyTrustChain = 0,
        AcceptUntrusted = 1,
    }
    impl TrustChainVerification {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TrustChainVerification::VerifyTrustChain => "VERIFY_TRUST_CHAIN",
                TrustChainVerification::AcceptUntrusted => "ACCEPT_UNTRUSTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERIFY_TRUST_CHAIN" => Some(Self::VerifyTrustChain),
                "ACCEPT_UNTRUSTED" => Some(Self::AcceptUntrusted),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericSecret {
    #[prost(message, optional, tag = "1")]
    pub secret: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdsSecretConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub sds_config: ::core::option::Option<
        super::super::super::super::config::core::v3::ConfigSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "secret::Type", tags = "2, 3, 4, 5")]
    pub r#type: ::core::option::Option<secret::Type>,
}
/// Nested message and enum types in `Secret`.
pub mod secret {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "2")]
        TlsCertificate(super::TlsCertificate),
        #[prost(message, tag = "3")]
        SessionTicketKeys(super::TlsSessionTicketKeys),
        #[prost(message, tag = "4")]
        ValidationContext(super::CertificateValidationContext),
        #[prost(message, tag = "5")]
        GenericSecret(super::GenericSecret),
    }
}
