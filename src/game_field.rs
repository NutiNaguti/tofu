use crate::{object::Object, vector::Vector};

pub struct GameField<'a> {
    width: u8,
    height: u8,
    objects: Vec<Object<'a>>,
    content: Vec<Vec<String>>,
}

impl<'a, 'b, 'c> GameField<'a> {
    pub fn create_game_field(x: u8, y: u8) -> GameField<'a> {
        GameField {
            width: x,
            height: y,
            objects: Vec::new(),
            content: Vec::from(Vec::new()),
        }
    }

    pub fn content(&self) -> &Vec<Vec<String>> {
        let content = String::new();

        &self.content
    }

    pub fn add_object(&mut self, object: Object<'a>) {
        // self.objects.push(object);
        // let content = String::new();
        // let
        //

        // self.update_content(&content);
    }

    fn update_content(&mut self, content: &str) {}

    fn size_changed() {}

    fn init_color_mask() {}

    fn update_color_mask() {}
}
