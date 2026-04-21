use crate::version::versions::ProtoVersion;
use bedrockrs_macros::packet;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::{Read, Write};

#[packet(id = 52)]
#[derive(Clone, Debug)]
pub struct CraftingDataPacket<V: ProtoVersion> {
    pub crafting_entries: Vec<V::CraftingDataEntryType>,
    pub potion_mixes: Vec<V::PotionMixDataEntry>,
    pub container_mixes: Vec<V::ContainerMixDataEntry>,
    pub material_reducers: Vec<V::MaterialReducerDataEntry>,
    pub clear_recipes: bool,
}

impl<V: ProtoVersion> ProtoCodec for CraftingDataPacket<V> {
    fn serialize<W: Write>(&self, stream: &mut W) -> Result<(), ProtoCodecError> {
        <u32 as ProtoCodecVAR>::serialize(&self.crafting_entries.len().try_into()?, stream)?;
        for entry in &self.crafting_entries {
            entry.serialize(stream)?;
        }

        <u32 as ProtoCodecVAR>::serialize(&self.potion_mixes.len().try_into()?, stream)?;
        for mix in &self.potion_mixes {
            mix.serialize(stream)?;
        }

        <u32 as ProtoCodecVAR>::serialize(&self.container_mixes.len().try_into()?, stream)?;
        for mix in &self.container_mixes {
            mix.serialize(stream)?;
        }

        <u32 as ProtoCodecVAR>::serialize(&self.material_reducers.len().try_into()?, stream)?;
        for reducer in &self.material_reducers {
            reducer.serialize(stream)?;
        }

        self.clear_recipes.serialize(stream)?;
        Ok(())
    }

    fn deserialize<R: Read>(stream: &mut R) -> Result<Self, ProtoCodecError> {
        let entry_count = <u32 as ProtoCodecVAR>::deserialize(stream)?;
        let mut crafting_entries = Vec::with_capacity(entry_count.try_into()?);
        for _ in 0..entry_count {
            crafting_entries.push(V::CraftingDataEntryType::deserialize(stream)?);
        }

        let potion_count = <u32 as ProtoCodecVAR>::deserialize(stream)?;
        let mut potion_mixes = Vec::with_capacity(potion_count.try_into()?);
        for _ in 0..potion_count {
            potion_mixes.push(V::PotionMixDataEntry::deserialize(stream)?);
        }

        let container_count = <u32 as ProtoCodecVAR>::deserialize(stream)?;
        let mut container_mixes = Vec::with_capacity(container_count.try_into()?);
        for _ in 0..container_count {
            container_mixes.push(V::ContainerMixDataEntry::deserialize(stream)?);
        }

        let reducer_count = <u32 as ProtoCodecVAR>::deserialize(stream)?;
        let mut material_reducers = Vec::with_capacity(reducer_count.try_into()?);
        for _ in 0..reducer_count {
            material_reducers.push(V::MaterialReducerDataEntry::deserialize(stream)?);
        }

        let clear_recipes = bool::deserialize(stream)?;

        Ok(Self {
            crafting_entries,
            potion_mixes,
            container_mixes,
            material_reducers,
            clear_recipes,
        })
    }

    fn size_hint(&self) -> usize {
        4 + self.crafting_entries.iter().map(|e| e.size_hint()).sum::<usize>()
            + 4 + self.potion_mixes.iter().map(|e| e.size_hint()).sum::<usize>()
            + 4 + self.container_mixes.iter().map(|e| e.size_hint()).sum::<usize>()
            + 4 + self.material_reducers.iter().map(|e| e.size_hint()).sum::<usize>()
            + self.clear_recipes.size_hint()
    }
}

