use super::super::enums::AimAssistAction;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 321)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistInstructionPacket {
    pub preset_id: String,
    pub action: AimAssistAction,
    pub allow_aim_assist: bool,
}
