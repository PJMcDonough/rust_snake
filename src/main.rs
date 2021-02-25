use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
};
use std::io::{stdout, Write};
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(_x: i32, _y: i32) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

struct Snake {
    head: Coord,
    tail: Vec<Coord>,
    length_to_add: i32,
}

impl Snake {
    fn new(pos: Coord) -> Self {
        Self{
            head: pos,
            tail: vec!(),
            length_to_add: 0,
        }
    }

    fn draw_head(){
    }
}

struct Game {
    board_size: Coord,
    snake: Snake,
    fruit: Coord,
}

impl Game {
    fn new() -> Self {
        Self{
            board_size: Coord::new(10, 10),
            snake: Snake::new(Coord::new(5, 5)),
            fruit: Coord::new(0, 0),
        }
    }
}

fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y));
}

fn get_char() -> KeyCode {
    loop {
        match read() {
            Ok(Event::Key(event)) => break event.code,
            Err(_) => panic!("Could not read!"),
            _ => (),
        }
    }
}

fn game_to_screen(pos: Coord) -> Coord {
    Coord::new(pos.x * 2, pos.y)
}

fn screen_to_game(pos: Coord) -> Coord{
    Coord::new(pos.x / 2, pos.y)
}

fn main() {
    let game = Game::new();
}
