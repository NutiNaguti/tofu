use std::{usize, vec};

use crate::{object::Object, vector::Vector};

pub struct GameField {
    width: u8,
    height: u8,
    objects: Vec<Object>,
    content: Vec<Vec<char>>,
}

impl<'a, 'b, 'c> GameField {
    pub fn new(x: u8, y: u8) -> GameField {
        let content: Vec<Vec<char>> = vec![vec![' '; x as usize]; y as usize];

        GameField {
            width: x,
            height: y,
            objects: Vec::new(),
            content,
        }
    }

    pub fn content(&self) -> &Vec<Vec<char>> {
        &self.content
    }

    pub fn add_object(&mut self, object: Object) {
        if object.height() >= self.height {
            return;
        }
        if object.width() >= self.width {
            return;
        }
        let mut new_content: Vec<Vec<char>> = self.content.clone();
        let mut y = usize::from(object.position().y);
        let mut x = usize::from(object.position().x);

        let _x = x.clone();
        let _y = y.clone();

        for (i, e) in object.content().iter().enumerate() {
            for (j, el) in e.iter().enumerate() {
                // println!("x: {:?}\ty: {:?}", x, y);
                new_content[y][x] = el.clone();
                x = x + 1;
            }
            new_content[i].push('\n');
            y = y + 1;
            x = _x;
        }

        self.content = new_content;
    }

    fn update_content(&mut self, content: &str) {}

    fn size_changed() {}

    fn init_color_mask() {}

    fn update_color_mask() {}
}
