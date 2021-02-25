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

struct Snake {
    head: Coord,
    tail: Vec<Coord>,
}

struct Game {
    board_size: Coord,
    snake: Snake,
    fruit: Coord,
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
    print!("{:?}", get_char());
}
