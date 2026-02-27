use super::super::enums::ContainerEnumName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct FullContainerName {
    container: ContainerEnumName,
    #[endianness(le)]
    dynamic_id: i32,
}
