use crate::data::datapack::SerializableDatapack;

mod data;

fn main() {
    //let datapack_1_20_4 = Datapack::from_zip("data/1-20-4.zip").unwrap();
    let datapack_terralith = SerializableDatapack::from_zip("data/Terralith_1.20_v2.4.11.zip").unwrap();

    println!("{:?}", datapack_terralith);
}
