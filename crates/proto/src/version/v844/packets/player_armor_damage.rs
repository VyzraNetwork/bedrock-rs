use bedrockrs_macros::{ProtoCodec, gamepacket};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::Cursor;

#[gamepacket(id = 149)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerArmorDamagePacket {
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub list: Vec<PlayerArmorDamageEntry>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerArmorDamageEntry {
    pub slot: i8,
    #[endianness(le)]
    pub damage: i16,
}
