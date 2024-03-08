use crate::data::datapack::DatapackFormat;

pub trait Element {
    fn serialize(format: DatapackFormat) -> &'static str;
}