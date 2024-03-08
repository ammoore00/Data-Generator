use serde::{Deserialize, Serialize};
use crate::data::datapack::DatapackFormat::FORMAT26;

//////////////////////////////////
//------ Datapack Formats ------//
//////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DatapackFormat {
    FORMAT6 = 0,
    FORMAT7,
    FORMAT8,
    FORMAT9,
    FORMAT10,
    FORMAT12,
    FORMAT15,
    FORMAT18,
    FORMAT26,
    FORMAT34
}

impl DatapackFormat {
    fn get_version_range(&self) -> [(i32, i32); 2] {
        use DatapackFormat::*;
        match *self {
            FORMAT6 => [(16, 2), (16, 5)],
            FORMAT7 => [(17, 0), (17, 1)],
            FORMAT8 => [(18, 0), (18, 1)],
            FORMAT9 => [(18, 2), (18, 2)],
            FORMAT10 => [(19, 0), (19, 3)],
            FORMAT12 => [(19, 4), (19, 4)],
            FORMAT15 => [(20, 0), (20, 1)],
            FORMAT18 => [(20, 2), (20, 2)],
            FORMAT26 => [(20, 3), (20, 4)],
            FORMAT34 => [(20, 5), (20, 5)],
        }
    }
}

///////////////////////////////
//------ Datapack Info ------//
///////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct PackInfo {
    pack: Pack,
    overlays: Overlays
}

#[derive(Debug, Serialize, Deserialize)]
struct Pack {
    pack_format: i32,
    #[serde(default)]
    supported_formats: Option<FormatRange>,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Overlays {
    entries: Vec<OverlayFormatEntry>
}

#[derive(Debug, Serialize, Deserialize)]
struct OverlayFormatEntry {
    formats: FormatRange,
    directory: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FormatRange {
    Exact(i32),
    Range((i32, i32)),
    Object {
        min_inclusive: i32,
        max_include: i32
    }
}