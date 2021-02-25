use crossterm::{execute, cursor::MoveTo};
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

fn main() {
}
