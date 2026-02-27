use super::super::enums::CameraAimAssistOperation;
use super::super::types::{CameraAimAssistCategories, CameraAimAssistPresetDefinition};
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 320)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistPresetsPacket {
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub categories: Vec<CameraAimAssistCategories>,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub presets: Vec<CameraAimAssistPresetDefinition>,
    pub operation: CameraAimAssistOperation,
}
