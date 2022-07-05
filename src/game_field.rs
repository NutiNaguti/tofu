use crate::{object::Object, vector::Vector};

pub struct GameField<'a> {
    width: u8,
    height: u8,
    objects: Vec<Object<'a>>,
    content: String,
}

impl<'a> GameField<'a> {
    pub fn create_game_field<'b>(x: u8, y: u8) -> GameField<'a> {
        GameField {
            width: x,
            height: y,
            objects: Vec::new(),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &String {
        let content = String::new();

        &self.content
    }

    pub fn add_object(&mut self, object_content: &str, position: Vector) {}

    fn update_content() {}

    fn size_changed() {}

    fn init_color_mask() {}

    fn update_color_mask() {}
}
