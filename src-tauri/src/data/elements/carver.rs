use serde::{Deserialize, Serialize};
use crate::data::datapack::DatapackFormat;
use crate::data::datapack::DatapackFormat::{FORMAT10, FORMAT12, FORMAT15, FORMAT18, FORMAT26, FORMAT6, FORMAT7, FORMAT8, FORMAT9};
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;

#[derive(Debug)]
pub struct CarverElement {

}

impl CarverElement {

}

impl NamedDataElement for CarverElement {
    fn serialize(&self, format: DatapackFormat) -> String {
        use crate::data::datapack::DatapackFormat::*;
        match format {
            FORMAT6 => {}
            FORMAT7 => {}
            FORMAT8 | FORMAT9 => {}
            FORMAT10 | FORMAT12 | FORMAT15 | FORMAT18 | FORMAT26 | FORMAT34 => {}
        }
        todo!()
    }

    fn deserialize(name: ResourceLocation, format: DatapackFormat, json: String) -> serde_json::Result<Box<Self>> {
        todo!()
    }

    fn add_data(&mut self, format: DatapackFormat, json: String) {
        todo!()
    }
}

/////////////////////////////////////
//------ Carver Data Storage ------//
/////////////////////////////////////

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CarverData {

}