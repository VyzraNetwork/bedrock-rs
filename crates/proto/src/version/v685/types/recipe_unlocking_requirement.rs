use crate::version::versions::ProtoVersion;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR};
use std::io::{Read, Write};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum UnlockingContext {
    None = 0,
    AlwaysUnlocked = 1,
    PlayerInWater = 2,
    PlayerHasManyItems = 3,
}

#[derive(Clone, Debug)]
pub struct RecipeUnlockingRequirement<V: ProtoVersion> {
    pub context: UnlockingContext,
    pub unlocking_ingredients: Vec<V::RecipeIngredient>,
}

impl<V: ProtoVersion> ProtoCodec for RecipeUnlockingRequirement<V> {
    fn serialize<W: Write>(&self, stream: &mut W) -> Result<(), ProtoCodecError> {
        (self.context as u8).serialize(stream)?;
        if self.context == UnlockingContext::None {
            let unlocking_ingredients = &self.unlocking_ingredients;
            <u32 as ProtoCodecVAR>::serialize(&unlocking_ingredients.len().try_into()?, stream)?;
            for ingredient in unlocking_ingredients {
                ingredient.serialize(stream)?;
            }
        }
        Ok(())
    }

    fn deserialize<R: Read>(stream: &mut R) -> Result<Self, ProtoCodecError> {
        let context = match u8::deserialize(stream)? {
            0 => UnlockingContext::None,
            1 => UnlockingContext::AlwaysUnlocked,
            2 => UnlockingContext::PlayerInWater,
            3 => UnlockingContext::PlayerHasManyItems,
            id => return Err(ProtoCodecError::InvalidEnumID(id.to_string(), "UnlockingContext")),
        };

        let unlocking_ingredients = if context == UnlockingContext::None {
            let count = <u32 as ProtoCodecVAR>::deserialize(stream)?;
            let mut unlocking_ingredients = Vec::with_capacity(count.try_into()?);
            for _ in 0..count {
                unlocking_ingredients.push(V::RecipeIngredient::deserialize(stream)?);
            }
            unlocking_ingredients
        } else {
            Vec::new()
        };

        Ok(Self {
            context,
            unlocking_ingredients,
        })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>()
            + if self.context == UnlockingContext::None {
                size_of::<u32>()
                    + self
                        .unlocking_ingredients
                        .iter()
                        .map(|i| i.size_hint())
                        .sum::<usize>()
            } else {
                0
            }
    }
}
