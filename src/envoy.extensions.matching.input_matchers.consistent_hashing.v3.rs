// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConsistentHashing {
    #[prost(uint32, tag = "1")]
    pub threshold: u32,
    #[prost(uint32, tag = "2")]
    pub modulo: u32,
    #[prost(uint64, tag = "3")]
    pub seed: u64,
}
