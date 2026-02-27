use super::super::types::FullContainerName;
use super::super::types::ItemStackResponseSlotInfo;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ItemStackResponseContainerInfo {
    pub container_name: FullContainerName,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub slots: Vec<ItemStackResponseSlotInfo>,
}
