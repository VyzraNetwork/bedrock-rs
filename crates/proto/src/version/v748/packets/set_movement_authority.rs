use super::super::enums::AuthoritativeMovementMode;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 319)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct SetMovementAuthorityPacket {
    pub movement_mode: AuthoritativeMovementMode,
}
