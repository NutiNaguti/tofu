use asset::Asset;
use std::env;

use crate::asset::AssetStorage;

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
    let mut amogus_list: Vec<String> = Vec::new();
    for e in &amogus_asset.content {
        amogus_list.push(String::from_iter(e));
    }

    let amogus = amogus_list.join("\n");
    println!("{amogus}");
    let mut asset_storage = AssetStorage::new();
    asset_storage.add_to_storage(&amogus_asset);
}
