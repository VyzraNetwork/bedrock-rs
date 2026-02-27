use super::super::types::FullContainerName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub container_name: FullContainerName,
    pub slot: i8,
    #[endianness(var)]
    pub raw_id: i32,
}
