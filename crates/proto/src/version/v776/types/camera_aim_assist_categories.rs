use super::super::types::CameraAimAssistCategory;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistCategories {
    pub identifier: String,
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub categories: Vec<CameraAimAssistCategory>,
}
