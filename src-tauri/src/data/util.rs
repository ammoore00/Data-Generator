use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    name: ResourceLocation,
    // TODO: Safer way of handling block states
    #[serde(rename = "Properties")]
    properties: Option<Value>
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    id: ResourceLocation,
    #[serde(rename = "Count")]
    count: i32,
    tag: ItemNBT
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemNBT {
    Format34 {
        // TODO: implement new item component data standard
    },
    // TODO: Safer way of handling item NBT
    Format26(Value)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ResourceLocation {
    // TODO: custom serialization logic
    namespace: String,
    id: String
}