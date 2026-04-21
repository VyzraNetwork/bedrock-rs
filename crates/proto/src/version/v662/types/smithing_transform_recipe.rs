use crate::version::versions::ProtoVersion;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct SmithingTransformRecipe<V: ProtoVersion> {
    pub recipe_id: Vec<u8>,
    pub template_ingredient: V::RecipeIngredient,
    pub base_ingredient: V::RecipeIngredient,
    pub addition_ingredient: V::RecipeIngredient,
    pub result: V::NetworkItemInstanceDescriptor,
    pub tag: Vec<u8>,
    #[endianness(var)]
    pub network_id: u32,
}
