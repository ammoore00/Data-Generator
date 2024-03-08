use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    name: ResourceLocation,
    // TODO: Safer way of handling block states
    #[serde(rename = "Properties")]
    properties: Option<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemNBT {
    // TODO: implement
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLocation {
    // TODO: custom serialization logic
    namespace: String,
    id: String
}