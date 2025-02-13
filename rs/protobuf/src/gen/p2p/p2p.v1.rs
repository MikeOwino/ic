#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateSyncId {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateSyncChunkRequest {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<StateSyncId>,
    #[prost(uint32, tag = "2")]
    pub chunk_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateSyncChunkResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertUpdate {
    #[prost(uint64, tag = "1")]
    pub commit_id: u64,
    #[prost(uint64, tag = "2")]
    pub slot_id: u64,
    #[prost(oneof = "advert_update::Update", tags = "3, 4")]
    pub update: ::core::option::Option<advert_update::Update>,
}
/// Nested message and enum types in `AdvertUpdate`.
pub mod advert_update {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        #[prost(bytes, tag = "3")]
        Artifact(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "4")]
        Advert(super::Advert),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Advert {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub attribute: ::prost::alloc::vec::Vec<u8>,
}
