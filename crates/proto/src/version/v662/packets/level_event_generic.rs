use std::io::{Read, Write};
use indexmap::IndexMap;
use crate::version::versions::ProtoVersion;
use bedrockrs_macros::packet;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[packet(id = 124)]
#[derive(Clone, Debug)]
pub struct LevelEventGenericPacket<V: ProtoVersion> {
    pub event_id: V::LevelEvent,
    pub event_data: IndexMap<String, nbtx::Value>,
}

impl<V: ProtoVersion> ProtoCodec for LevelEventGenericPacket<V> {
    fn serialize<W: Write>(&self, stream: &mut W) -> Result<(), ProtoCodecError> {
        self.event_id.serialize(stream)?;

        let encoded = nbtx::to_bytes::<nbtx::NetworkLittleEndian>(&self.event_data)?;
        if encoded.len() < 3 || encoded[0] != 10 || encoded[1] != 0 || *encoded.last().unwrap() != 0 {
            return Err(ProtoCodecError::FormatMismatch("Invalid rooted NBT encoding for LevelEventGenericPacket"));
        }

        // Packet 124 uses headless NBT compound content (without root type/name).
        stream.write_all(&encoded[2..])?;

        Ok(())
    }

    fn deserialize<R: Read>(stream: &mut R) -> Result<Self, ProtoCodecError> {
        let event_id = V::LevelEvent::deserialize(stream)?;

        let mut headless = Vec::new();
        stream.read_to_end(&mut headless)?;

        // Wrap headless payload into a synthetic root compound so nbtx can decode it.
        let mut wrapped = Vec::with_capacity(headless.len() + 2);
        wrapped.push(10);
        wrapped.push(0);
        wrapped.extend_from_slice(&headless);

        let mut cursor = std::io::Cursor::new(wrapped);
        let event_data =
            nbtx::from_bytes::<nbtx::NetworkLittleEndian, IndexMap<String, nbtx::Value>>(&mut cursor)?;

        Ok(Self {
            event_id,
            event_data,
        })
    }

    fn size_hint(&self) -> usize {
        self.event_id.size_hint() + 1
    }
}
