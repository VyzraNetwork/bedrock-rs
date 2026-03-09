use bedrockrs_macros::{ProtoCodec, gamepacket};
use vek::Vec3;

#[gamepacket(id = 331)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct GraphicsParameterOverridePacket {
    #[vec_repr(i32)]
    #[vec_endianness(var)]
    pub values: Vec<GraphicsParameterOverrideKeyFrame>,
    pub biome_identifier: String,
    pub parameter_type: GraphicsParameterOverrideType,
    pub reset: bool,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct GraphicsParameterOverrideKeyFrame {
    #[endianness(le)]
    pub key: f32,
    #[endianness(le)]
    pub value: Vec3<f32>,
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum GraphicsParameterOverrideType {
    SkyZenithColor = 0,
    SkyHorizonColor = 1,
    HorizonBlendMin = 2,
    HorizonBlendMax = 3,
    HorizonBlendStart,
    HorizonBlendMieStart,
    RayleighStrength,
    SunMieStrength,
    MoonMieStrength,
    SunGlareShape,
    Chlorophyll,
    CDOM,
    SuspendedSediment,
    WavesDepth,
    WavesFrequency,
    WavesFrequencyScaling,
    WavesSpeed,
    WavesSpeedScaling,
    WavesShape,
    WavesOctaves,
    WavesMix,
    WavesPull,
    WavesDirectionIncrement,
    MidtonesContrast,
    HighlightsContrast,
    ShadowsContrast,
}
