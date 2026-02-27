use super::super::types::CameraAimAssistItemSettings;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistPresetDefinition {
    pub identifier: String,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub exclusion_list: Vec<String>,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub liquid_targeting_list: Vec<String>,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub item_settings: Vec<CameraAimAssistItemSettings>,
    pub default_item_settings: Option<String>,
    pub hand_settings: Option<String>,
}
