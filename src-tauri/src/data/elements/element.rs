use crate::data::datapack::DatapackFormat;
use crate::data::util::ResourceLocation;

pub trait NamedDataElement {
    fn serialize(&self, format: DatapackFormat) -> &'static str;
    fn deserialize(name: ResourceLocation, format: DatapackFormat, json: &str) -> serde_json::Result<Box<Self>>;
    fn add_data(&mut self, format: DatapackFormat, json: &str);
}

pub trait AnonymousDataElement {
    fn serialize(&self, format: DatapackFormat) -> &'static str;
    fn deserialize(format: DatapackFormat, json: &str) -> serde_json::Result<Box<Self>>;
    fn add_data(&mut self, format: DatapackFormat, json: &str);
}