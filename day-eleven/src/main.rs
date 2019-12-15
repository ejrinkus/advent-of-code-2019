use std::collections::HashMap;
extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(Clone, Copy, FromPrimitive)]
enum Color {
    BLACK = 0,
    WHITE = 1,
}

#[derive(Clone, Copy)]
enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn move_robo(pos: &mut (i32, i32), dir: &Dir) {
    match dir {
        Dir::UP => pos.1 -= 1,
        Dir::RIGHT => pos.0 += 1,
        Dir::DOWN => pos.1 += 1,
        Dir::LEFT => pos.0 -= 1,
    }
}

fn rotate_robo(dir: &mut Dir, val: i32) {
    if val != 0 && val != 1 { panic!("Invalid rotation value {}!", val); }

    match dir {
        Dir::UP => if val == 0 { *dir = Dir::LEFT; } else { *dir = Dir::RIGHT; },
        Dir::RIGHT => if val == 0 { *dir = Dir::UP; } else { *dir = Dir::DOWN; },
        Dir::DOWN => if val == 0 { *dir = Dir::RIGHT; } else { *dir = Dir::LEFT; },
        Dir::LEFT => if val == 0 { *dir = Dir::DOWN; } else { *dir = Dir::UP; },
    }
}

fn get_robo_output(robo: &mut intcode::IntcodeComp) -> Option<(Color, u8)> {
    let maybe_color = robo.pop_output();
    let maybe_dir = robo.pop_output();
    if maybe_color == None && maybe_dir == None {
        return None;
    } else if maybe_dir == None {
        panic!("Output should always come in pairs!");
    }
    let color = match num::FromPrimitive::from_i64(maybe_color.unwrap()) {
        Some(Color::BLACK) => Color::BLACK,
        Some(Color::WHITE) => Color::WHITE,
        _ => panic!("Unexpected color value {:?}!", maybe_color.unwrap()),
    };
    return Some((color, maybe_dir.unwrap() as u8));
}

fn get_paint_job(min_x: i32, max_x: i32, min_y: i32, max_y: i32, panels: HashMap<(i32, i32), Color>) {
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            match panels.get(&(x, y)) {
                Some(color) => {
                    match color {
                        Color::BLACK => print!(". "),
                        Color::WHITE => print!("# "),
                    }
                },
                None => {
                    if x == 0 && y == 0 {
                        // The origin is the only panel that started white.
                        // If the origin is not in the panels map, that means
                        // we never repainted it.
                        print!("# ");
                    } else {
                        print!(". ");
                    }
                },
            }
        }
        print!("\n");
    }
}

fn main() {
    let line = std::fs::read_to_string("day-eleven/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);

    let mut robo_pos: (i32, i32) = (0, 0);
    let mut robo_dir: Dir = Dir::UP;
    let mut panels: HashMap<(i32, i32), Color> = HashMap::new();
    let mut robo = intcode::IntcodeComp::new(tape);
    let mut first_panel = true;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    robo.start();
    while *robo.state() != intcode::IntcodeState::Finished {
        match robo.state() {
            intcode::IntcodeState::NeedsInput => {
                let mut cur_color = Color::BLACK;
                if first_panel {
                    cur_color = Color::WHITE;
                    first_panel = false;
                }
                match panels.get(&robo_pos) {
                    Some(color) => cur_color = *color,
                    None => (),
                }
                robo.push_input(cur_color as i64);
                robo.start();
            },
            intcode::IntcodeState::Err(s) => panic!("Unexpected error: {}", s),
            _ => (),
        }
        // robo.start() blocks until the computer either finishes or needs
        // input.  Either way, we should take the opportunity to update any
        // panels based on any output the computer has generated up until
        // this point.
        let mut maybe_output = get_robo_output(&mut robo);
        loop {
            if maybe_output.is_none() {
                break;
            }
            let (color, dir) = maybe_output.unwrap();
            panels.insert(robo_pos.clone(), color);
            rotate_robo(&mut robo_dir, dir as i32);
            maybe_output = get_robo_output(&mut robo);
        }
        move_robo(&mut robo_pos, &robo_dir);
        if robo_pos.0 < min_x {
            min_x = robo_pos.0;
        }
        if robo_pos.0 > max_x {
            max_x = robo_pos.0;
        }
        if robo_pos.1 < min_y {
            min_y = robo_pos.1;
        }
        if robo_pos.1 > max_y {
            max_y = robo_pos.1;
        }
    }

    println!("Painted {} panels", panels.len());
    get_paint_job(min_x, max_x, min_y, max_y, panels);
}
