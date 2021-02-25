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
}

impl Snake {
    fn new(pos: Coord) -> Self {
        Self{
            head: pos,
            tail: vec!(),
        }
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

fn main() {
    let game = Game::new();
}
