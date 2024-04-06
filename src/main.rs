use crate::data::datapack::Datapack;

mod data;

fn main() {
    println!("Hello, world!");

    let datapack_1_20_4 = Datapack::from_zip("data/1-20-4.zip").unwrap();

    println!("{:?}", datapack_1_20_4);
}
