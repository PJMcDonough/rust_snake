use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{stdout, Write};
use std::time::Duration;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(_x: i32, _y: i32) -> Self {
        Self { x: _x, y: _y }
    }
}

struct Snake {
    head: Coord,
    tail: Vec<Coord>,
    length_to_add: i32,
    facing: Direction,
    alive: bool,
}

impl Snake {
    fn new(pos: Coord) -> Self {
        Self {
            head: pos,
            tail: vec![],
            length_to_add: 0,
            facing: Direction::Up,
            alive: true,
        }
    }

    fn draw_head(&self) {
        let real_pos = game_to_screen(self.head);
        execute!(
            stdout(),
            MoveTo(real_pos.x as u16, real_pos.y as u16),
            SetBackgroundColor(Color::Blue),
            SetForegroundColor(Color::Yellow),
            Print("()"),
            ResetColor
        )
        .unwrap();
    }

    fn draw_tail(&self) {
        for cell in self.tail.iter() {
            let real_pos = game_to_screen(*cell);
            execute!(
                stdout(),
                MoveTo(real_pos.x as u16, real_pos.y as u16),
                SetBackgroundColor(Color::Blue),
                SetForegroundColor(Color::Yellow),
                Print("XX"),
                ResetColor
            )
            .unwrap();
        }
    }

    fn draw(&self) {
        self.draw_head();
        self.draw_tail();
    }

    fn update(&mut self, board_size: &Coord) {
        self.tail.insert(0, self.head);
        match self.facing {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        };
        if self.head.y < 0
            || self.head.y >= board_size.y
            || self.head.x < 0
            || self.head.x >= board_size.x
        {
            self.alive = false;
            return;
        }
        self.draw();
        if self.length_to_add != 0 {
            self.length_to_add -= 1;
        } else {
            self.tail.pop();
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
        Self {
            board_size: Coord::new(40, 40),
            snake: Snake::new(Coord::new(5, 5)),
            fruit: Coord::new(0, 0),
        }
    }

    fn run(&mut self) -> crossterm::Result<()> {
        loop {
            self.draw_board();
            self.snake.update(&self.board_size);
            if !self.snake.alive {
                break;
            }
            while poll(Duration::from_millis(0))? {
                match read()? {
                    Event::Key(event) => {
                        let direction = match event.code {
                            KeyCode::Up => Some(Direction::Up),
                            KeyCode::Down => Some(Direction::Down),
                            KeyCode::Left => Some(Direction::Left),
                            KeyCode::Right => Some(Direction::Right),
                            _ => None,
                        };
                        match direction {
                            Some(dir) => self.snake.facing = dir,
                            None => (),
                        }
                    }
                    _ => (),
                }
            }
            std::thread::sleep(Duration::from_millis(200));
        }
        Ok(())
    }

    fn draw_board(&self) {
        execute!(stdout(), MoveTo(0, 0));
        for i in 0..(self.board_size.x + 2) {
            print!("XX");
        }
        println!();
        for i in 0..self.board_size.x {
            print!("XX");
            for j in 0..self.board_size.y {
                print!("  ");
            }
            println!("XX");
        }
        for i in 0..(self.board_size.x + 2) {
            print!("XX");
        }
    }
}

fn move_cursor(pos: Coord) {
    execute!(stdout(), MoveTo(pos.x as u16, pos.y as u16)).unwrap();
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
    Coord::new(pos.x * 2 + 2, pos.y + 1)
}

fn screen_to_game(pos: Coord) -> Coord {
    Coord::new(pos.x / 2 - 1, pos.y - 1)
}

fn main() {
    execute!(stdout(), Hide).unwrap();
    let mut game = Game::new();
    game.run();
    execute!(stdout(), Show).unwrap();
}
