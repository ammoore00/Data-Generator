use serde::{Deserialize, Serialize};
use tauri::regex::Regex;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;

#[derive(Debug)]
pub struct CarverElement {

}

impl CarverElement {

}

impl NamedDataElement for CarverElement {
    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(name: ResourceLocation, json: String) -> serde_json::Result<Box<Self>> {
        todo!()
    }

    fn get_file_regex() -> &'static Regex {
        todo!()
    }
}

/////////////////////////////////////
//------ Carver Data Storage ------//
/////////////////////////////////////

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CarverData {

}