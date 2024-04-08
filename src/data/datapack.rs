use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use regex::Regex;
use strum_macros::FromRepr;
use zip::read::ZipFile;
use zip::result::ZipError;
use zip::ZipArchive;
use crate::data::datapack::DatapackFormat::FORMAT18;
use crate::data::biome::BiomeSerializableData;
use crate::data::util::{ResourceLocation, Text};

//////////////////////////////////
//------ Datapack Formats ------//
//////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr, FromRepr)]
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
    FORMAT35 = 35
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
            FORMAT35 => [(20, 5), (20, 5)],
        }
    }

    pub fn supports_overlays(&self) -> bool {
        *self >= Self::get_minimum_overlay_version()
    }

    pub fn get_minimum_overlay_version() -> Self {
        FORMAT18
    }
}

///////////////////////////////////
//------ Datapack Handling ------//
///////////////////////////////////

lazy_static! {
    static ref NAMESPACE_REG: Regex = Regex::new(r"data/([a-z0-9_.-]+)").unwrap();
    static ref DATA_REG: Regex = Regex::new(r"data/.+").unwrap();
    static ref OVERLAY_REG: Regex = Regex::new(r"^([a-z0-9_-]+)/data/(.+)").unwrap();
}

//------------//

#[derive(Debug)]
pub struct SerializableDatapack {
    pub pack_info: SerializablePackInfo,

    biomes: HashMap<ResourceLocation, Box<SerializableDataHolder<BiomeSerializableData>>>
}

impl SerializableDatapack {
    fn empty(pack_info: SerializablePackInfo) -> Self {
        SerializableDatapack {
            pack_info,

            biomes: HashMap::new()
        }
    }

    //------ File Handling ------//

    pub fn from_zip(filepath: &str) -> Result<SerializableDatapack, DatapackError> {
        let zip_file = File::open(&filepath)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // Load pack info
        let mut pack_info_str = String::new();
        archive.by_name("pack.mcmeta")?.read_to_string(&mut pack_info_str)?;
        let pack_info: SerializablePackInfo = serde_json::from_str(pack_info_str.clone().as_ref())?;

        let mut datapack = SerializableDatapack::empty(pack_info.clone());

        // Iterate through files in the archive
        // Must be done by index instead of iterator to allow mutable access to contents
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let data_source = Self::get_data_source(&mut file, &datapack)?;
            Self::import_data(&mut file, &mut datapack, data_source)?;
        }

        Ok(datapack)
    }

    fn get_data_source(file: &mut ZipFile, datapack: &SerializableDatapack) -> Result<SerializableDataSource, DatapackError> {
        let name = file.name().to_owned();

        let mut data_source = SerializableDataSource::Root;
        let mut overlay_directory: Option<&str> = None;

        // Is the current file part of the base data or the overlay?
        if let Some(overlay_cap) = OVERLAY_REG.captures(&*name) {
            // If it is in an overlay, either get the existing overlay or make a new one with the
            // overlay folder name obtained from the file path and then set that as the target data holder
            overlay_directory = Some(overlay_cap.get(1).unwrap().as_str());

            if let Some(overlay) = datapack.pack_info.get_overlay(overlay_directory.unwrap()) {
                data_source = SerializableDataSource::Overlay(overlay);
            }
            else {
                return Err(DatapackError::Overlay(format!("Overlay directory {} found, but not declared in pack info!", overlay_directory.unwrap())))
            }
        }

        Ok(data_source)
    }

    fn import_data(file: &mut ZipFile, datapack: &mut SerializableDatapack, data_source: SerializableDataSource) -> Result<(), DatapackError> {
        let name = file.name();

        // Makes sure all files in data folder are in a valid namespace
        if let Some(cap) = NAMESPACE_REG.captures(name) {
            let namespace = cap.get(1).unwrap().clone().as_str();

            if let Some(cap) = BiomeSerializableData::get_file_regex().captures(file.name()) {
                let id = cap.get(1).unwrap().clone().as_str();
                let resource_location = ResourceLocation::new(String::from(namespace), String::from(id));

                //println!("{}", name);
                println!("{}", resource_location);

                let mut biome_data = String::new();
                file.read_to_string(&mut biome_data)?;
                //println!("{}", biome_data);
                let biome = *BiomeSerializableData::deserialize(biome_data.clone())?;

                match datapack.biomes.entry(resource_location.clone()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().add(data_source, Box::new(biome));
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(SerializableDataHolder::named(resource_location, data_source, biome));
                    }
                }
            }
        }
        // Only error on data reg match to ignore files outside the data folder (and the data folder itself)
        else if let Some(_) = DATA_REG.find(name) {
            return Err(DatapackError::Namespace(format!("Invalid namespace in \"{}\"! Only a-z, 0-9, '_', '-', '.' are allowed!", name)))
        }

        Ok(())
    }

    // TODO: Writing back into a zip file
}

//////////////////////////////////////////
//------ Serialized Datapack Info ------//
//////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePackInfo {
    pack: SerializablePackData,
    #[serde(default)]
    overlays: Option<SerializablePackOverlays>
}

impl SerializablePackInfo {
    fn get_overlay(&self, name: &str) -> Option<SerializableOverlayEntry> {
        if let Some(overlays) = &self.overlays {
            return overlays.get_overlay(name)
        }

        None
    }
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializablePackData {
    pack_format: DatapackFormat,
    #[serde(default)]
    supported_formats: Option<FormatRange>,
    description: PackDescription
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializablePackOverlays {
    entries: Vec<SerializableOverlayEntry>
}

impl SerializablePackOverlays {
    fn get_overlay(&self, name: &str) -> Option<SerializableOverlayEntry> {
        for overlay in &self.entries {
            if overlay.directory == name {
                return Some(overlay.clone())
            }
        }

        None
    }
}

//------------//

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct SerializableOverlayEntry {
    formats: FormatRange,
    directory: String
}

//------------//

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum FormatRange {
    Exact(i32),
    Range((i32, i32)),
    Object {
        min_inclusive: i32,
        max_inclusive: i32
    }
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum PackDescription {
    Text(Text),
    Array(Vec<Text>)
}

/////////////////////////////////////////
//------ Serialized Data Storage ------//
/////////////////////////////////////////

#[derive(Debug, Clone)]
enum SerializableDataSource {
    Root,
    Overlay(SerializableOverlayEntry)
}

//------------//

#[derive(Debug)]
pub struct SerializableDataHolder<T: SerializableDataElement> {
    root_data: Option<T>,
    overlay_data: HashMap<SerializableOverlayEntry, T>,
    resource_location: Option<ResourceLocation>
}

impl<T: SerializableDataElement> SerializableDataHolder<T> {
    pub fn anonymous(data_source: SerializableDataSource, data: T) -> Box<Self> {
        match data_source {
            SerializableDataSource::Root => {
                Box::new(SerializableDataHolder {
                    root_data: Some(data),
                    overlay_data: HashMap::new(),
                    resource_location: None
                })
            }
            SerializableDataSource::Overlay(overlay) => {
                let mut data_holder = Box::new(SerializableDataHolder {
                    root_data: None,
                    overlay_data: HashMap::new(),
                    resource_location: None
                });

                data_holder.overlay_data.insert(overlay, data);

                return data_holder
            }
        }
    }

    pub fn named(resource_location: ResourceLocation, data_source: SerializableDataSource, data: T) -> Box<Self> {
        let mut data_holder = Self::anonymous(data_source, data);
        data_holder.resource_location = Some(resource_location);
        data_holder
    }

    pub fn add(&mut self, data_source: SerializableDataSource, data_element: Box<T>) {
        match data_source {
            SerializableDataSource::Root => {
                self.root_data = Some(*data_element);
            }
            SerializableDataSource::Overlay(overlay) => {
                self.overlay_data.insert(overlay, *data_element);
            }
        }
    }

    pub fn get(&self, data_source: SerializableDataSource) -> Option<&T> {
        match data_source {
            SerializableDataSource::Root => {
                Option::from(&self.root_data)
            }
            SerializableDataSource::Overlay(overlay) => {
                self.overlay_data.get(&overlay)
            }
        }
    }

    pub fn remove(&mut self, data_source: SerializableDataSource) {
        match data_source {
            SerializableDataSource::Root => {
                self.root_data = None;
            }
            SerializableDataSource::Overlay(overlay) => {
                self.overlay_data.remove(&overlay);
            }
        }
    }
}

//------------//

pub trait DataHandler<T: SerializableDataElement> {
    fn from_serializable_data_holder(data_holder: SerializableDataHolder<T>) -> Self where Self: Sized;
    fn into_serializable_data_holder(self) -> SerializableDataHolder<T>;
}

//------------//

pub trait SerializableDataElement {
    fn serialize(&self) -> String;
    fn deserialize(json: String) -> serde_json::Result<Box<Self>> where Self: Sized;
}

//------------//

pub trait FileElement : SerializableDataElement {
    fn get_file_regex() -> &'static Regex;
}

///////////////////////////////////////
//------ Internal Data storage ------//
///////////////////////////////////////

pub struct Overlay {
    name: String,
    min_format: DatapackFormat,
    max_format: DatapackFormat
}

impl Overlay {
    fn from_single_format(name: String, format: DatapackFormat) -> Result<Self, DatapackError> {
        Self::from_formats(name, format, format)
    }

    fn from_formats(name: String, min_format: DatapackFormat, max_format: DatapackFormat) -> Result<Self, DatapackError> {
        if min_format > max_format {
            return Err(DatapackError::Overlay(format!("Minimum format {} cannot be greater than maximum format {}", min_format as u8, max_format as u8)))
        }

        if !min_format.supports_overlays() && min_format != max_format {
            return Err(DatapackError::Overlay(format!("Format ranges are not supported below format {}", DatapackFormat::get_minimum_overlay_version() as u8)))
        }

        Ok(Overlay {
            name,
            min_format,
            max_format
        })
    }

    fn from_single_int_format(name: String, format_int: i32) -> Result<Self, DatapackError> {
        Self::from_int_formats(name, format_int, format_int)
    }

    fn from_int_formats(name: String, min_format_int: i32, max_format_int: i32) -> Result<Self, DatapackError> {
        let min_format = DatapackFormat::from_repr(min_format_int as u8).ok_or(DatapackError::Format(format!("Invalid datapack format {min_format_int}")))?;
        let max_format = DatapackFormat::from_repr(max_format_int as u8).ok_or(DatapackError::Format(format!("Invalid datapack format {max_format_int}")))?;

        Ok(Overlay {
            name,
            min_format,
            max_format
        })
    }
}

impl TryFrom<SerializableOverlayEntry> for Overlay {
    type Error = DatapackError;

    fn try_from(value: SerializableOverlayEntry) -> Result<Self, Self::Error> {
        match value.formats {
            FormatRange::Exact(version) => {
                Overlay::from_int_formats(value.directory, version, version)
            }
            FormatRange::Range((min_version, max_version)) => {
                Overlay::from_int_formats(value.directory, min_version, max_version)
            }
            FormatRange::Object {min_inclusive, max_inclusive} => {
                Overlay::from_int_formats(value.directory, min_inclusive, max_inclusive)
            }
        }
    }
}

//------------//

pub trait DataEntry<T> {
    fn get_name(&self) -> &str;
    //fn get_values(&self) -> HashMap<>;
}

//------------//

pub struct DataValue<T> {
    name: String,
    value: T
}

/////////////////////////////
//------ Error Types ------//
/////////////////////////////

#[derive(Debug)]
pub enum DatapackError {
    File(String),
    Deserialize(String),
    Namespace(String),
    Overlay(String),
    Format(String)
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