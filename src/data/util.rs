use serde_with::skip_serializing_none;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;
use crate::data::datapack::DatapackError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    name: ResourceLocation,
    // TODO: Safer way of handling block states
    #[serde(rename = "Properties")]
    properties: Option<Value>
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    id: ResourceLocation,
    #[serde(rename = "Count")]
    count: i32,
    tag: ItemNBT
}

//------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemNBT {
    Format35 {
        // TODO: implement new item component data standard
    },
    // TODO: Safer way of handling item NBT
    Format26(Value)
}

//------------//

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceLocation {
    // TODO: custom serialization logic
    namespace: String,
    id: String
}

impl ResourceLocation {
    pub(crate) fn new(namespace: String, id: String) -> Self {
        ResourceLocation { namespace, id }
    }
}

lazy_static! {
    static ref RESOURCE_LOCATION_REG: Regex = Regex::new(r"^[a-z0-9_.-]+:[a-z0-9_.-]+$").unwrap();
}

impl FromStr for ResourceLocation {
    type Err = ResourceLocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if RESOURCE_LOCATION_REG.find(s).is_some() {
            let split: Vec<String> = s.split(":").map(|s| s.to_string()).collect();

            Ok(ResourceLocation {
                namespace: split.get(0).ok_or_else(|| Self::Err::Parse(format!("Missing first split on ':' in {}", s)))?.clone(),
                id: split.get(1).ok_or_else(|| Self::Err::Parse(format!("Missing second split on ':' in {}", s)))?.clone()
            })
        }
        else { Err(Self::Err::Syntax(format!("Resource location \"{}\" does not match namespace requirements! a-z, 0-9, '_', '-', '.' are allowed characters, and a single ':' must separate the namespace from the id", s))) }
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", &self.namespace, &self.id))
    }
}

//------------//

#[derive(Debug)]
pub enum ResourceLocationError {
    Regex(String),
    Parse(String),
    Syntax(String)
}

impl From<regex::Error> for ResourceLocationError {
    fn from(value: regex::Error) -> Self {
        ResourceLocationError::Regex(format!("Error Creating regex: {}", value.to_string()))
    }
}

//------------//

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializableText {
    String(String),
    List(Vec<SerializableText>),
    Object {
        #[serde(default)]
        text: Option<String>,
        #[serde(default)]
        translate: Option<String>,
        // Not implemented: score, selector, keybind, nbt:
        //    these are not relevant to world gen, but may become
        //    relevant if application scope expands to include more
        //    datapack elements
        #[serde(default)]
        extra: Option<Vec<SerializableText>>,
        #[serde(default)]
        color: Option<String>,
        #[serde(default)]
        font: Option<ResourceLocation>,
        #[serde(default)]
        bold: Option<bool>,
        #[serde(default)]
        italic: Option<bool>,
        #[serde(default)]
        underlined: Option<bool>,
        #[serde(default)]
        strikethrough: Option<bool>,
        #[serde(default)]
        obfuscated: Option<bool>,
        // Interactivity not implemented as it does not apply to world gen
    }
}

//------------//

lazy_static! {
    static ref DEFAULT_FONT: ResourceLocation = ResourceLocation::from_str("minecraft:default").unwrap();
}

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    extra: Vec<Text>,
    should_translate: bool,

    color: Option<Color>,
    font: Option<ResourceLocation>,

    is_bold: bool,
    is_italic: bool,
    is_underlined: bool,
    is_strikethrough: bool,
    is_obfuscated: bool,
}

impl Text {
    fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
            extra: Vec::new(),
            should_translate: false,

            color: None,
            font: None,

            is_bold: false,
            is_italic: false,
            is_underlined: false,
            is_strikethrough: false,
            is_obfuscated: false,
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new("")
    }
}

impl TryFrom<Vec<SerializableText>> for Text {
    type Error = DatapackError;

    fn try_from(mut value: Vec<SerializableText>) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Err(DatapackError::Deserialize("Text as list cannot be empty".parse().unwrap())),
            1 => Ok(Self::try_from(value.remove(0))?),
            _ => {
                let ser_root = value.remove(0);
                let root = Self::try_from(ser_root)?;

                let mut err = None;
                let extra = value.into_iter().map(|ser_text| {
                    match Self::try_from(ser_text) {
                        Ok(txt) => Some(txt),
                        Err(e) => {
                            err = Some(e);
                            None
                        }
                    }
                })
                .filter_map(|t| t)
                .collect();

                if let Some(e) = err { return Err(e) }

                Ok(Self {
                    extra,
                    .. root
                })
            }
        }
    }
}

impl TryFrom<SerializableText> for Text {
    type Error = DatapackError;

    fn try_from(value: SerializableText) -> Result<Self, Self::Error> {
        match value {
            SerializableText::String(text) => {
                Ok(Self::new(&text))
            }
            SerializableText::List(list) => {
                Self::try_from(list)
            }
            SerializableText::Object { text, translate, extra, color,
                font, bold, italic, underlined, strikethrough, obfuscated
            } => {
                let mut should_translate = false;

                let text_opt = if text.is_some() { text }
                else if translate.is_some() {
                    should_translate = true;
                    translate
                }
                else { None };

                let text = text_opt.ok_or(DatapackError::Deserialize("Text object must have either 'text' or 'translate' fields".parse().unwrap()))?;

                let mut err = None;

                let extra_list: Vec<Text> = if let Some(extra) = extra {
                    extra.into_iter().map(|ser_text| {
                        match Self::try_from(ser_text) {
                            Ok(txt) => Some(txt),
                            Err(e) => {
                                err = Some(e);
                                None
                            }
                        }
                    })
                    .filter_map(|t| t)
                    .collect()
                } else { Vec::new() };

                if let Some(e) = err { return Err(e) }

                let color = if let Some(c) = color {
                    match Color::from_str(&*c) {
                        Ok(c) => Some(c),
                        Err(e) => {
                            err = Some(e);
                            None
                        }
                    }
                } else { None };

                if let Some(e) = err { return Err(e) }

                Ok(Self {
                    text,
                    extra: extra_list,
                    should_translate,

                    color,
                    font,

                    is_bold: bold.unwrap_or(false),
                    is_italic: italic.unwrap_or(false),
                    is_underlined: underlined.unwrap_or(false),
                    is_strikethrough: strikethrough.unwrap_or(false),
                    is_obfuscated: obfuscated.unwrap_or(false),
                })
            }
        }
    }
}

impl Into<SerializableText> for Text {
    fn into(self) -> SerializableText {
        todo!()
    }
}

//------------//

lazy_static! {
    static ref BLACK: Color = Color::Name(String::from("black"));
    static ref DARK_BLUE: Color = Color::Name(String::from("dark_blue"));
    static ref DARK_GREEN: Color = Color::Name(String::from("dark_green"));
    static ref DARK_AQUA: Color = Color::Name(String::from("dark_aqua"));
    static ref DARK_RED: Color = Color::Name(String::from("dark_red"));
    static ref DARK_PURPLE: Color = Color::Name(String::from("dark_purple"));
    static ref GOLD: Color = Color::Name(String::from("gold"));
    static ref GRAY: Color = Color::Name(String::from("gray"));
    static ref DARK_GRAY: Color = Color::Name(String::from("dark_gray"));
    static ref BLUE: Color = Color::Name(String::from("blue"));
    static ref GREEN: Color = Color::Name(String::from("green"));
    static ref AQUA: Color = Color::Name(String::from("aqua"));
    static ref RED: Color = Color::Name(String::from("red"));
    static ref LIGHT_PURPLE: Color = Color::Name(String::from("light_purple"));
    static ref YELLOW: Color = Color::Name(String::from("yellow"));
    static ref WHITE: Color = Color::Name(String::from("white"));
}

#[derive(Debug, Clone)]
pub enum Color {
    Hex(u32),
    Name(String)
}

impl Color {
    pub fn get_color(&self) -> Option<u32> {
        match self {
            Color::Hex(val) => Some(*val),
            Color::Name(name) => {
                Self::get_color_from_str(name)
            }
        }
    }

    fn get_color_from_str(name: &str) -> Option<u32> {
        match &*name {
            // Values obtained from MC wiki
            "black" => Some(0x000000),
            "dark_blue" => Some(0x0000AA),
            "dark_green" => Some(0x00AA00),
            "dark_aqua" => Some(0x00AAAA),
            "dark_red" => Some(0xAA0000),
            "dark_purple" => Some(0xAA00AA),
            "gold" => Some(0xFFAA00),
            "gray" => Some(0xAAAAAA),
            "dark_gray" => Some(0x555555),
            "blue" => Some(0x5555FF),
            "green" => Some(0x55FF55),
            "aqua" => Some(0x55FFFF),
            "red" => Some(0xFF5555),
            "light_purple" => Some(0xFF55FF),
            "yellow" => Some(0xFFFF55),
            "white" => Some(0xFFFFFF),
            _ => None
        }
    }
}

impl FromStr for Color {
    type Err = DatapackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<u32>() {
            Ok(Self::Hex(val))
        }
        else if let Some(_) = Self::get_color_from_str(s) {
            Ok(Self::Name(String::from(s)))
        }
        else {
            Err(DatapackError::Deserialize(format!("Unknown color name {s}")))
        }
    }
}