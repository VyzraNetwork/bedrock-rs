use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ProtoCodec for Color {
    fn serialize<W: Write>(&self, stream: &mut W) -> Result<(), ProtoCodecError> {
        <u32 as ProtoCodecLE>::serialize(
            &((self.a as u32)
                | ((self.r as u32) << 8)
                | ((self.g as u32) << 16)
                | ((self.b as u32) << 24)),
            stream,
        )?;
        Ok(())
    }

    fn deserialize<R: Read>(stream: &mut R) -> Result<Self, ProtoCodecError> {
        let v = <u32 as ProtoCodecLE>::deserialize(stream)?;
        Ok(Color {
            a: v as u8,
            r: (v >> 8) as u8,
            g: (v >> 16) as u8,
            b: (v >> 24) as u8,
        })
    }

    fn size_hint(&self) -> usize {
        size_of::<i32>()
    }
}
