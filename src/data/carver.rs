use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::data::datapack::{SerializableDataElement, DataHandler, FileElement};

#[derive(Debug)]
pub struct CarverElement {

}

impl CarverElement {

}

impl SerializableDataElement for CarverElement {
    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(json: String) -> serde_json::Result<Box<Self>> {
        todo!()
    }
}

impl FileElement for CarverElement {
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