use super::super::enums::{
    CodeBuilderCodeStatus, CodeBuilderStorageCategory, CodeBuilderStorageOperation,
};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 178)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CodeBuilderSourcePacket {
    pub operation: CodeBuilderStorageOperation,
    pub category: CodeBuilderStorageCategory,
    pub code_status: CodeBuilderCodeStatus,
}
