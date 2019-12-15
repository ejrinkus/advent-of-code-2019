#[macro_use] extern crate text_io;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum TileId {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

impl fmt::Display for TileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           TileId::EMPTY => write!(f, " "),
           TileId::WALL => write!(f, "-"),
           TileId::BLOCK => write!(f, "#"),
           TileId::PADDLE => write!(f, "="),
           TileId::BALL => write!(f, "O"),
       }
    }
}

fn render_screen(x_range: &(i64, i64), y_range: &(i64, i64), tiles: &HashMap<(i64, i64), TileId>) {
    for y in y_range.0..y_range.1+1 {
        for x in x_range.0..x_range.1+1 {
            match tiles.get(&(x, y)) {
                Some(tile) => print!("{}", tile),
                None => print!(" "),
            }
        }
        print!("\n");
    }
}

fn main() {
    let line = std::fs::read_to_string("day-thirteen/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    let mut arcade = intcode::IntcodeComp::new(tape);
    arcade.start();

    let mut tiles: HashMap<(i64, i64), TileId> = HashMap::new();
    let mut score = 0;
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut ball_pos = (0, 0);
    let mut paddle_pos = (0, 0);
    loop {
        loop {
            // Handle output to render screen.
            let maybe_x = arcade.pop_output();
            if maybe_x.is_none() { break; }
            let x: i64 = maybe_x.unwrap();
            let y: i64 = arcade.pop_output().unwrap();
            if x == -1 && y == 0 {
                score = arcade.pop_output().unwrap();
                continue;
            }
            let tile: TileId = match arcade.pop_output().unwrap() {
                0 => TileId::EMPTY,
                1 => TileId::WALL,
                2 => TileId::BLOCK,
                3 => {
                    paddle_pos = (x, y);
                    TileId::PADDLE
                },
                4 => {
                    ball_pos = (x, y);
                    TileId::BALL
                },
                _ => panic!("Unknown tile type!"),
            };
            if x < x_range.0 { x_range.0 = x; }
            if x > x_range.1 { x_range.1 = x; }
            if y < y_range.0 { y_range.0 = y; }
            if y > y_range.1 { y_range.1 = y; }
            tiles.insert((x, y), tile);
        }

        // Dumb input AI: just follow the ball.
        if *arcade.state() == intcode::IntcodeState::Finished { break; }
        if paddle_pos.0 < ball_pos.0 {
            arcade.push_input(1);
        } else if paddle_pos.0 > ball_pos.0 {
            arcade.push_input(-1);
        } else {
            arcade.push_input(0);
        }
        arcade.start();
    }
    render_screen(&x_range, &y_range, &tiles);
    println!("score: {}", score);
}
