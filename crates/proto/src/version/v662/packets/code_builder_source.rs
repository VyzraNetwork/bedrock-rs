use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::proto_version::ProtoVersion;

#[gamepacket(id = 178)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CodeBuilderSourcePacket<V: ProtoVersion> {
    pub operation: V::CodeBuilderStorageOperation,
    pub category: V::CodeBuilderStorageCategory,
    pub value: String,
}