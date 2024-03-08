use serde::{Deserialize, Serialize};
use crate::data::datapack::DatapackFormat;
use crate::data::datapack::DatapackFormat::{FORMAT10, FORMAT12, FORMAT15, FORMAT18, FORMAT26, FORMAT6, FORMAT7, FORMAT8, FORMAT9};
use crate::data::elements::element::DataElement;

#[derive(Debug)]
pub struct CarverElement {

}

impl CarverElement {

}

impl DataElement for CarverElement {
    fn serialize(&self, format: DatapackFormat) -> &'static str {
        use crate::data::datapack::DatapackFormat::*;
        match format {
            FORMAT6 => {}
            FORMAT7 => {}
            FORMAT8 | FORMAT9 => {}
            FORMAT10 | FORMAT12 | FORMAT15 | FORMAT18 | FORMAT26 | FORMAT34 => {}
        }
        todo!()
    }

    fn deserialize(&self, format: DatapackFormat, json: &str) {
        todo!()
    }
}

/////////////////////////////////////
//------ Carver Data Storage ------//
/////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct CarverData {

}