use crate::game_field::GameField;

pub fn display(scene: &GameField) {
    let mut result = String::new();
    for e in scene.content() {
        let temp: String = e.iter().collect();
        result.push_str(&temp);
    }
    println!("{}", result);
}
