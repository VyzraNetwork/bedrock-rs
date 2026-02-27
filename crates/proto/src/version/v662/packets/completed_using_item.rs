use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::proto_version::ProtoVersion;

#[gamepacket(id = 142)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CompletedUsingItemPacket<V: ProtoVersion> {
    #[endianness(le)]
    pub item_id: u16,
    pub item_use_method: V::ItemUseMethod,
}