use ex::fs;
use std::{io::Error, process};

pub struct AssetStorage<'a, 'b> {
    pub assets: Vec<&'a Asset<'b>>,
}

pub struct Asset<'c> {
    pub name: &'c str,
    pub path: &'c str,
    pub content: Vec<Vec<char>>,
}

fn parse_from_file(path: &str) -> Result<String, ex::io::Error> {
    dbg!(path);
    let file_content = fs::read_to_string(path);
    file_content
}

impl<'c> Asset<'c> {
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

    pub fn width(&self) -> u8 {
        self.content.len() as u8
    }

    //TODO
    pub fn height(&self) -> u8 {
        self.content[0].len() as u8
    }
}

impl<'a, 'b> AssetStorage<'a, 'b> {
    pub fn new() -> AssetStorage<'a, 'b> {
        AssetStorage { assets: Vec::new() }
    }

    pub fn add(&mut self, asset: &'a Asset<'b>) {
        self.assets.push(asset);
    }

    pub fn get_asset_by_name(&self, name: &str) -> Option<&Asset<'_>> {
        for e in &self.assets {
            if e.name == name {
                return Some(e);
            }
        }

        return None;
    }
}
