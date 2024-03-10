use std::fs::File;
use std::io;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tauri::regex::{Captures, Regex};
use zip::result::ZipError;
use zip::ZipArchive;
use crate::data::datapack::DatapackFormat::FORMAT18;
use crate::data::elements::biome::BiomeElement;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;
use crate::io::json_io::{get_zip_as_archive, read_file_from_archive};

//////////////////////////////////
//------ Datapack Formats ------//
//////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DatapackFormat {
    FORMAT6 = 6,
    FORMAT7 = 7,
    FORMAT8 = 8,
    FORMAT9 = 9,
    FORMAT10 = 10,
    FORMAT12 = 12,
    FORMAT15 = 15,
    FORMAT18 = 18,
    FORMAT26 = 26,
    FORMAT34 = 34
}

impl DatapackFormat {
    pub fn get_version_range(&self) -> [(i32, i32); 2] {
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

    pub fn supports_overlays(&self) -> bool {
        *self >= FORMAT18
    }
}

///////////////////////////////
//------ Datapack Info ------//
///////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
    pack: Pack,
    #[serde(default)]
    overlays: Option<Overlays>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pack {
    pack_format: DatapackFormat,
    #[serde(default)]
    supported_formats: Option<FormatRange>,
    description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Overlays {
    entries: Vec<OverlayFormatEntry>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverlayFormatEntry {
    formats: FormatRange,
    directory: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum FormatRange {
    Exact(i32),
    Range((i32, i32)),
    Object {
        min_inclusive: i32,
        max_include: i32
    }
}

///////////////////////////////////
//------ Datapack Handling ------//
///////////////////////////////////

#[derive(Debug, Clone)]
pub struct Datapack {
    pack_info: PackInfo,
    overlays: Vec<DatapackData>,
    data: DatapackData
}

lazy_static! {
    static ref NAMESPACE_REG: Regex = Regex::new(r"^data/([a-z0-9_.-]+)").unwrap();
    static ref DATA_REG: Regex = Regex::new(r"^data/.+").unwrap();
    static ref OVERLAY_REG: Regex = Regex::new(r"^overlays/([a-z0-9_-]+)/(.+)").unwrap();
}

impl Datapack {
    fn empty(pack_info: PackInfo) -> Self {
        Datapack {
            pack_info,
            overlays: Vec::new(),
            data: DatapackData::empty(DataSource::Root)
        }
    }

    pub fn from_zip(filepath: &str) -> Result<Datapack, DatapackError> {
        let mut archive = get_zip_as_archive(filepath)?;
        let pack_info_str = read_file_from_archive(&mut archive, "pack.mcmeta")?;
        let pack_info: PackInfo = serde_json::from_str(&*pack_info_str)?;

        let mut datapack = Datapack::empty(pack_info.clone());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut name = file.name();

            let data = &datapack.data;
            let mut overlay: Option<&str> = None;

            let overlap_cap = OVERLAY_REG.captures(name);

            if overlap_cap.is_some() {
                overlay = Some(get_string_from_capture(&overlap_cap.as_ref().unwrap(), 1));
                name = get_string_from_capture(&overlap_cap.as_ref().unwrap(), 2);
            }

            // Makes sure all files in data folder are in a valid namespace
            if let Some(cap) = NAMESPACE_REG.captures(name) {
                let namespace = get_string_from_capture(&cap, 1);

                // TODO: account for overlays - overlay data will be caught by the filename regexes
                //       so it must be filtered out beforehand
                if let Some(cap) = BiomeElement::get_file_regex().captures(name) {
                    let id = get_string_from_capture(&cap, 1);
                    let resource_location = ResourceLocation::new(String::from(namespace), String::from(id));

                    println!("{}", resource_location);

                    let mut biome_data = String::new();
                    file.read_to_string(&mut biome_data)?;
                    let biome = *BiomeElement::deserialize(resource_location, &pack_info.pack.pack_format, biome_data)?;

                    datapack.data.biomes.push(biome);
                }
            }
            // Ignore files outside the data folder (and the data folder itself)
            else if let Some(_) = DATA_REG.find(name) {
                return Err(DatapackError::Namespace(format!("Invalid namespace in \"{}\"! Only a-z, 0-9, '_', '-', '.' are allowed!", name)))
            }
        }

        Ok(datapack)
    }
}

#[derive(Debug, Clone)]
struct DatapackData {
    source: DataSource,

    biomes: Vec<BiomeElement>
}

impl DatapackData {
    fn empty(source: DataSource) -> Self {
        DatapackData {
            source,

            biomes: Vec::new()
        }
    }

    fn from_archive(archive: ZipArchive<File>, path: &str) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
enum DataSource {
    Root,
    Overlay(String)
}

#[derive(Debug)]
pub enum DatapackError {
    File(String),
    Deserialize(String),
    Namespace(String)
}

impl From<ZipError> for DatapackError {
    fn from(value: ZipError) -> Self {
        DatapackError::File(format!("Error reading zip file: {}", value.to_string()))
    }
}

impl From<io::Error> for DatapackError {
    fn from(value: io::Error) -> Self {
        DatapackError::File(format!("Error reading file from zip: {}", value.to_string()))
    }
}

impl From<serde_json::Error> for DatapackError {
    fn from(value: serde_json::Error) -> Self {
        DatapackError::Deserialize(format!("Error deserializing file: {}", value.to_string()))
    }
}

fn get_string_from_capture<'a>(cap: &'a Captures, index: i32) -> &'a str {
    cap.get(1).unwrap().as_str()
}