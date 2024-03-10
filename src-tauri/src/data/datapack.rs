use std::fmt::Formatter;
use std::io;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Error;
use tauri::regex::Regex;
use zip::result::ZipError;
use crate::data::datapack::DatapackFormat::{FORMAT10, FORMAT12, FORMAT15, FORMAT18, FORMAT26, FORMAT34, FORMAT6, FORMAT7, FORMAT8, FORMAT9};
use crate::data::elements::biome::BiomeElement;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;
use crate::io::json_io::{get_zip_as_archive, read_file_from_archive};

//////////////////////////////////
//------ Datapack Formats ------//
//////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
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

    fn from_format_number(number: u64) -> Option<Self> {
        match number {
            6 => Some(FORMAT6),
            7 => Some(FORMAT7),
            8 => Some(FORMAT8),
            9 => Some(FORMAT9),
            10 => Some(FORMAT10),
            12 => Some(FORMAT12),
            15 => Some(FORMAT15),
            18 => Some(FORMAT18),
            26 => Some(FORMAT26),
            34 => Some(FORMAT34),
            _ => None
        }
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
    #[serde(deserialize_with = "deserialize_pack_format")]
    pack_format: DatapackFormat,
    #[serde(default)]
    supported_formats: Option<FormatRange>,
    description: String
}

fn deserialize_pack_format<'de, D>(deserializer: D) -> Result<DatapackFormat, D::Error>
where D: Deserializer<'de> {
    struct PackFormatVisitor;

    impl<'de> de::Visitor<'de> for PackFormatVisitor {
        type Value = DatapackFormat;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("an integer matching a valid pack format")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where E: Error {
            DatapackFormat::from_format_number(v).ok_or(E::custom("Invalid pack format number"))
        }
    }

    deserializer.deserialize_u64(PackFormatVisitor)
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
    overlays: Vec<DatapackFormat>,

    biomes: Vec<BiomeElement>
}

lazy_static! {
    static ref NAMESPACE_REG: Regex = Regex::new(r"^data/([a-z0-9_.-]+)").unwrap();
    static ref DATA_REG: Regex = Regex::new(r"^data/.+").unwrap();
}

impl Datapack {
    fn empty(pack_info: PackInfo) -> Self {
        Datapack {
            pack_info,
            overlays: Vec::new(),

            biomes: Vec::new()
        }
    }

    pub fn from_zip(filepath: &str) -> Result<Datapack, DatapackError> {
        let mut archive = get_zip_as_archive(filepath)?;
        let pack_info_str = read_file_from_archive(&mut archive, "pack.mcmeta")?;
        let pack_info: PackInfo = serde_json::from_str(&*pack_info_str)?;

        let mut datapack = Datapack::empty(pack_info.clone());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name();

            // Makes sure all files in data folder are in a valid namespace
            if let Some(cap) = NAMESPACE_REG.captures(name) {
                let namespace = cap.get(1).unwrap().as_str().parse().unwrap();

                if let Some(cap) = BiomeElement::get_file_regex().captures(name) {
                    let id: String = cap.get(1).unwrap().as_str().parse().unwrap();
                    let resource_location = ResourceLocation::new(namespace, id);

                    println!("{}", resource_location);

                    let mut biome_data = String::new();
                    file.read_to_string(&mut biome_data)?;
                    let biome = *BiomeElement::deserialize(resource_location, &pack_info.pack.pack_format, biome_data)?;

                    datapack.biomes.push(biome);
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