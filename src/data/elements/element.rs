use regex::Regex;

pub trait DataElement {
    fn serialize(&self) -> String;
    fn deserialize(json: String) -> serde_json::Result<Box<Self>>;
}

pub trait FileElement {
    fn get_file_regex() -> &'static Regex;
}