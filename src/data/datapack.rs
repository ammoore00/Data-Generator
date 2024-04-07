use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use regex::Regex;
use zip::read::ZipFile;
use zip::result::ZipError;
use zip::ZipArchive;
use crate::data::datapack::DatapackFormat::FORMAT18;
use crate::data::elements::biome::BiomeElement;
use crate::data::elements::element::{DataElement, FileElement};
use crate::data::util::{ResourceLocation, Text};

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

///////////////////////////////////
//------ Datapack Handling ------//
///////////////////////////////////

lazy_static! {
    static ref NAMESPACE_REG: Regex = Regex::new(r"data/([a-z0-9_.-]+)").unwrap();
    static ref DATA_REG: Regex = Regex::new(r"data/.+").unwrap();
    static ref OVERLAY_REG: Regex = Regex::new(r"^([a-z0-9_-]+)/data/(.+)").unwrap();
}

//------------//

#[derive(Debug, Clone)]
pub struct Datapack {
    pub pack_info: PackInfo,

    biomes: HashMap<ResourceLocation, DataHolder<BiomeElement>>
}

impl Datapack {
    fn empty(pack_info: PackInfo) -> Self {
        Datapack {
            pack_info,

            biomes: HashMap::new()
        }
    }

    pub fn from_zip(filepath: &str) -> Result<Datapack, DatapackError> {
        let zip_file = File::open(&filepath)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // Load pack info
        let mut pack_info_str = String::new();
        archive.by_name("pack.mcmeta")?.read_to_string(&mut pack_info_str)?;
        let pack_info: PackInfo = serde_json::from_str(pack_info_str.clone().as_ref())?;

        let mut datapack = Datapack::empty(pack_info.clone());

        // Iterate through files in the archive
        // Must be done by index instead of iterator to allow mutable access to contents
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let data_source = Self::get_data_source(&mut file, &datapack)?;
            Self::import_data(&mut file, &mut datapack, data_source)?;
        }

        Ok(datapack)
    }

    fn get_data_source(file: &mut ZipFile, datapack: &Datapack) -> Result<DataSource, DatapackError> {
        let name = file.name().to_owned();

        let mut data_source = DataSource::Root;
        let mut overlay_directory: Option<&str> = None;

        // Is the current file part of the base data or the overlay?
        if let Some(overlay_cap) = OVERLAY_REG.captures(&*name) {
            // If it is in an overlay, either get the existing overlay or make a new one with the
            // overlay folder name obtained from the file path and then set that as the target data holder
            overlay_directory = Some(overlay_cap.get(1).unwrap().as_str());

            if let Some(overlay) = datapack.pack_info.get_overlay(overlay_directory.unwrap()) {
                data_source = DataSource::Overlay(overlay);
            }
            else {
                return Err(DatapackError::Overlay(format!("Overlay directory {} found, but not declared in pack info!", overlay_directory.unwrap())))
            }
        }

        Ok(data_source)
    }

    fn import_data(file: &mut ZipFile, datapack: &mut Datapack, data_source: DataSource) -> Result<(), DatapackError> {
        let name = file.name();

        // Makes sure all files in data folder are in a valid namespace
        if let Some(cap) = NAMESPACE_REG.captures(name) {
            let namespace = cap.get(1).unwrap().clone().as_str();

            if let Some(cap) = BiomeElement::get_file_regex().captures(file.name()) {
                let id = cap.get(1).unwrap().clone().as_str();
                let resource_location = ResourceLocation::new(String::from(namespace), String::from(id));

                println!("{}", name);
                println!("{}", resource_location);

                let mut biome_data = String::new();
                file.read_to_string(&mut biome_data)?;
                //println!("{}", biome_data);
                let biome = *BiomeElement::deserialize(biome_data.clone())?;

                match datapack.biomes.entry(resource_location.clone()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().add(data_source, Box::new(biome));
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(DataHolder::named(resource_location, data_source, biome));
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
}

///////////////////////////////
//------ Datapack Info ------//
///////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
    pack: Pack,
    #[serde(default)]
    overlays: Option<PackOverlays>
}

impl PackInfo {
    fn get_overlay(&self, name: &str) -> Option<Overlay> {
        if let Some(overlays) = &self.overlays {
            return overlays.get_overlay(name)
        }

        None
    }
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pack {
    pack_format: DatapackFormat,
    #[serde(default)]
    supported_formats: Option<FormatRange>,
    description: PackDescription
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackOverlays {
    entries: Vec<Overlay>
}

impl PackOverlays {
    fn get_overlay(&self, name: &str) -> Option<Overlay> {
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
struct Overlay {
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
        max_include: i32
    }
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum PackDescription {
    Text(Text),
    Array(Vec<Text>)
}

//////////////////
// Data Storage //
//////////////////

#[derive(Debug, Clone)]
enum DataSource {
    Root,
    Overlay(Overlay)
}

//------------//

#[derive(Debug, Clone)]
pub struct DataHolder<T>
where T: DataElement {
    root_data: Option<T>,
    data: HashMap<Overlay, T>,
    resource_location: Option<ResourceLocation>
}

impl<T: DataElement> DataHolder<T> {
    pub fn anonymous(data_source: DataSource, data: T) -> Self {
        match data_source {
            DataSource::Root => {
                DataHolder {
                    root_data: Some(data),
                    data: HashMap::new(),
                    resource_location: None
                }
            }
            DataSource::Overlay(overlay) => {
                let mut data_holder = DataHolder {
                    root_data: None,
                    data: HashMap::new(),
                    resource_location: None
                };

                data_holder.data.insert(overlay, data);

                return data_holder
            }
        }
    }

    pub fn named(resource_location: ResourceLocation, data_source: DataSource, data: T) -> Self {
        let mut data_holder = Self::anonymous(data_source, data);
        data_holder.resource_location = Some(resource_location);
        data_holder
    }

    pub fn add(&mut self, data_source: DataSource, data_element: Box<T>) {
        match data_source {
            DataSource::Root => {
                self.root_data = Some(*data_element);
            }
            DataSource::Overlay(overlay) => {
                self.data.insert(overlay, *data_element);
            }
        }
    }

    pub fn get(&self, data_source: DataSource) -> Option<&T> {
        match data_source {
            DataSource::Root => {
                Option::from(&self.root_data)
            }
            DataSource::Overlay(overlay) => {
                self.data.get(&overlay)
            }
        }
    }

    pub fn remove(&mut self, data_source: DataSource) {
        match data_source {
            DataSource::Root => {
                self.root_data = None;
            }
            DataSource::Overlay(overlay) => {
                self.data.remove(&overlay);
            }
        }
    }
}

/////////////////
// Error Types //
/////////////////

#[derive(Debug)]
pub enum DatapackError {
    File(String),
    Deserialize(String),
    Namespace(String),
    Overlay(String)
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