mod direction;
mod display;
mod game_field;
mod object;
mod vector;

fn main() {
    // println!("Hello, world!");
    game_field::GameField::create_game_field(60, 60);
}
