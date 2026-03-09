use bedrockrs_macros::ProtoCodec;
use vek::Vec2;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum PredictionType {
    Player = 0,
    Vehicle = 1,
}
