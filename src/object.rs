use crate::{direction::Direction, vector::Vector};
use termion::color::{self, Color};

#[derive(Debug)]
pub struct Object<'a> {
    width: u8,
    height: u8,
    color: &'a mut dyn Color,
    position: Vector,
    content: Vec<Vec<String>>,
}

impl<'a> Object<'a> {
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

    pub fn color(&self) -> &dyn Color {
        self.color
    }

    pub fn set_color_rgb(&mut self, new_color: &'a mut color::Rgb) {
        self.color = new_color;
    }

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
        color: &'a mut color::Rgb,
        position: Vector,
        content: Vec<Vec<String>>,
    ) -> Object<'a> {
        Object {
            width,
            height,
            color,
            position: Vector::zero(),
            content,
        }
    }
}
