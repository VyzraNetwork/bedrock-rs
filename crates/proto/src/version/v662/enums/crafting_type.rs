use bedrockrs_macros::ProtoCodec;

#[deprecated]
#[derive(ProtoCodec, Clone, Debug)]
#[enum_endianness(var)]
#[enum_repr(u32)]
#[repr(u32)]
pub enum CraftingType {
    Inventory = 0,
    Crafting = 1,
}
