use crate::data::datapack::DatapackFormat;

pub trait DataElement {
    fn serialize(&self, format: DatapackFormat) -> &'static str;
    fn deserialize(format: DatapackFormat, json: &str);
    fn add_data(&self, format: DatapackFormat, json: &str);
}

pub trait VersionedData<T>
where T: Copy {
    fn get_value(&self, format: DatapackFormat) -> Option<T>;
}