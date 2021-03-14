use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{stdout, Write};
use std::time::Duration;
use rand::seq::SliceRandom;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
            || self.tail.iter().any(|&pos| pos == self.head)
        {
            self.alive = false;
            return;
        }
        if self.length_to_add != 0 {
            self.length_to_add -= 1;
        } else {
            self.tail.pop();
        }
        self.draw();
    }
}

struct Game {
    board_size: Coord,
    snake: Snake,
    fruit: Coord,
}

impl Game {
    fn new() -> Self {
        let mut s = Self {
            board_size: Coord::new(40, 40),
            snake: Snake::new(Coord::new(5, 5)),
            fruit: Coord::new(0, 0),
        };
        s.set_random_fruit();
        s
    }

    fn set_random_fruit(&mut self){
        let mut v = Vec::new();
        for i in 0..self.board_size.x {
            for j in 0..self.board_size.y {
                v.push(Coord::new(i, j));
            }
        }
        let mut i = 0;
        while i != v.len() {
            if self.snake.head == v[i]
                || self.snake.tail.iter().any(|&pos| pos == v[i])
            {
                v.remove(i);
            } else {
                i += 1;
            }
        }
        self.fruit = *v.choose(&mut rand::thread_rng()).unwrap();
    }

    fn draw_fruit(&self) {
        let real_pos = game_to_screen(self.fruit);
        execute!(
            stdout(),
            MoveTo(real_pos.x as u16, real_pos.y as u16),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::Green),
            Print("()"),
            ResetColor
            )
            .unwrap();
    }

    fn run(&mut self) -> crossterm::Result<()> {
        loop {
            self.draw_board();
            self.draw_fruit();
            self.snake.update(&self.board_size);
            if !self.snake.alive {
                break;
            } else if self.snake.head == self.fruit {
                self.set_random_fruit();
                self.snake.length_to_add += 2;
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
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn draw_board(&self) {
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        for _i in 0..(self.board_size.x + 2) {
            print!("XX");
        }
        println!("\r");
        for _i in 0..self.board_size.x {
            print!("XX");
            for _j in 0..self.board_size.y {
                print!("  ");
            }
            println!("XX\r");
        }
        for _i in 0..(self.board_size.x + 2) {
            print!("XX");
        }
    }
}

fn game_to_screen(pos: Coord) -> Coord {
    Coord::new(pos.x * 2 + 2, pos.y + 1)
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    let mut game = Game::new();
    game.run().unwrap();
    execute!(stdout(), Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}
