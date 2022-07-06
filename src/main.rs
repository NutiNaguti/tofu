use asset::Asset;
use std::env;

use crate::{asset::AssetStorage, object::Object, vector::Vector};

mod asset;
mod direction;
mod display;
mod game_field;
mod object;
mod vector;

fn main() {
    // println!("Hello, world!");
    // game_field::GameField::create_game_field(60, 60);
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let amogus_asset = Asset::new(&filename);
    // let mut amogus_list: Vec<String> = Vec::new();
    // for e in &amogus_asset.content {
    //     amogus_list.push(String::from_iter(e));
    // }

    // let amogus = amogus_list.join("\n");
    let mut asset_storage = AssetStorage::new();
    asset_storage.add(&amogus_asset);
    let mut scene = game_field::GameField::new(100, 50);
    // println!("{}", amogus_asset.name);
    let object =
        Object::create_from_asset_storage(&asset_storage, "amogus.txt", Vector::new(0, 0)).unwrap();
    // println!("object: {:?}", object);
    scene.add_object(object);
    display::display(&scene);
}
