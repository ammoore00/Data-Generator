use tauri::regex::Regex;
use crate::data::datapack::DatapackFormat;
use crate::data::util::ResourceLocation;

pub trait NamedDataElement {
    fn serialize(&self, format: &DatapackFormat) -> String;
    fn deserialize(name: ResourceLocation, format: &DatapackFormat, json: String) -> serde_json::Result<Box<Self>>;
    fn add_data(&mut self, format: &DatapackFormat, json: String);
    fn get_file_regex() -> &'static Regex;
}

pub trait AnonymousDataElement {
    fn serialize(&self, format: &DatapackFormat) -> String;
    fn deserialize(format: &DatapackFormat, json: String) -> serde_json::Result<Box<Self>>;
    fn add_data(&mut self, format: &DatapackFormat, json: String);
}