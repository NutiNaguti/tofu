use ex::fs;
use std::{io::Error, process};

pub struct AssetStorage<'a, 'b> {
    pub assets: Vec<&'a Asset<'b>>,
}

pub struct Asset<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub content: Vec<Vec<char>>,
}

fn parse_from_file(path: &str) -> Result<String, ex::io::Error> {
    dbg!(path);
    let file_content = fs::read_to_string(path);
    file_content
}

impl<'a> Asset<'a> {
    pub fn new(path: &str) -> Asset {
        let (_, name) = path.split_once("/").unwrap();
        let raw_content = parse_from_file(path).unwrap_or_else(|err| {
            dbg!("{}", err);
            process::exit(1);
        });

        let mut content: Vec<Vec<char>> = Vec::from(Vec::new());
        let lines: Vec<&str> = raw_content.lines().collect();

        for (i, e) in lines.iter().enumerate() {
            let char_list: Vec<char> = e.chars().collect();
            content.push(char_list);
        }

        Asset {
            name,
            path,
            content,
        }
    }
}

impl<'a, 'b> AssetStorage<'a, 'b> {
    pub fn new() -> AssetStorage<'a, 'b> {
        AssetStorage { assets: Vec::new() }
    }

    pub fn add_to_storage(&mut self, asset: &'a Asset<'b>) {
        self.assets.push(asset);
    }

    // pub fn get_asset_by_name(&self, name: &str) -> &Asset {
    //     for (i, e) in self.assets.iter().enumerate() {
    //         if e. == name {
    //             return &e;
    //         }
    //     }
    // }
}
