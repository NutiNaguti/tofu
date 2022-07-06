use crate::{
    asset::{Asset, AssetStorage},
    direction::Direction,
    vector::Vector,
};
use termion::color::{self, Rgb};

#[derive(Debug)]
pub struct Object {
    width: u8,
    height: u8,
    color: Rgb,
    position: Vector,
    content: Vec<Vec<char>>,
}

const WHITE: Rgb = color::Rgb(255u8, 255u8, 255u8);

impl<'a> Object {
    pub fn x(&self) -> u8 {
        self.position.x
    }

    pub fn y(&self) -> u8 {
        self.position.y
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn color(&self) -> &Rgb {
        &self.color
    }

    pub fn content(&self) -> &Vec<Vec<char>> {
        &self.content
    }

    pub fn position(&self) -> &Vector {
        &self.position
    }

    // pub fn set_color_rgb(&mut self, new_color: &'a mut color::Rgb) {
    //     self.color = new_color;
    // }

    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            &Direction::Up => self.position = Vector::new(self.x(), self.y() + 1),
            &Direction::Down => self.position = Vector::new(self.x(), self.y() - 1),
            &Direction::Left => self.position = Vector::new(self.x() - 1, self.y()),
            &Direction::Right => self.position = Vector::new(self.x() + 1, self.y()),
            _ => (),
        }
    }

    pub fn new(
        width: u8,
        height: u8,
        color: Rgb,
        position: Vector,
        content: Vec<Vec<char>>,
    ) -> Object {
        Object {
            width,
            height,
            color,
            position,
            content,
        }
    }

    //TODO
    pub fn create_from_asset_storage(
        storage: &AssetStorage,
        name: &str,
        position: Vector,
    ) -> Option<Object> {
        let result = storage
            .get_asset_by_name(name)
            .expect("create_from_asset_storage error");
        let result = Some(Object::new(
            result.width(),
            result.height(),
            WHITE,
            position,
            result.content.clone(),
        ));

        // let assets = storage.get_asset_by_name(name);
        // let mut color = color::Rgb(0u8, 0u8, 0u8);
        // let assets = assets.into_iter().map(|x| {
        //     result.push(Object::new(
        //         x.width().clone(),
        //         x.height().clone(),
        //         &mut color,
        //         Vector::zero(),
        //         x.content.clone(),
        //     ))
        // });
        result
    }
}
