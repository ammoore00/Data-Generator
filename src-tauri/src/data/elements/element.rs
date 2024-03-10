use tauri::regex::Regex;
use crate::data::util::ResourceLocation;

pub trait NamedDataElement {
    fn serialize(&self) -> String;
    fn deserialize(name: ResourceLocation, json: String) -> serde_json::Result<Box<Self>>;
    fn get_file_regex() -> &'static Regex;
}

pub trait AnonymousDataElement {
    fn serialize(&self) -> String;
    fn deserialize(json: String) -> serde_json::Result<Box<Self>>;
}