use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    name: ResourceLocation,
    // TODO: Safer way of handling block states
    #[serde(rename = "Properties")]
    properties: Option<Value>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    id: ResourceLocation,
    #[serde(rename = "Count")]
    count: i32,
    tag: ItemNBT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemNBT {
    Format34 {
        // TODO: implement new item component data standard
    },
    // TODO: Safer way of handling item NBT
    Format26(Value)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLocation {
    // TODO: custom serialization logic
    namespace: String,
    id: String
}

impl ResourceLocation {
    pub(crate) fn new(namespace: String, id: String) -> Self {
        ResourceLocation { namespace, id }
    }
}

lazy_static! {
    static ref RESOURCE_LOCATION_REG: Regex = Regex::new(r"^[a-z0-9_.-]+:[a-z0-9_.-]+$").unwrap();
}

impl FromStr for ResourceLocation {
    type Err = ResourceLocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if RESOURCE_LOCATION_REG.find(s).is_some() {
            let split: Vec<String> = s.split(":").map(|s| s.to_string()).collect();

            Ok(ResourceLocation {
                namespace: split.get(0).ok_or_else(|| Self::Err::Parse(format!("Missing first split on ':' in {}", s)))?.clone(),
                id: split.get(1).ok_or_else(|| Self::Err::Parse(format!("Missing second split on ':' in {}", s)))?.clone()
            })
        }
        else { Err(Self::Err::Syntax(format!("Resource location \"{}\" does not match namespace requirements! a-z, 0-9, '_', '-', '.' are allowed characters, and a single ':' must separate the namespace from the id", s))) }
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", &self.namespace, &self.id))
    }
}

#[derive(Debug)]
pub enum ResourceLocationError {
    Regex(String),
    Parse(String),
    Syntax(String)
}

impl From<tauri::regex::Error> for ResourceLocationError {
    fn from(value: tauri::regex::Error) -> Self {
        ResourceLocationError::Regex(format!("Error Creating regex: {}", value.to_string()))
    }
}