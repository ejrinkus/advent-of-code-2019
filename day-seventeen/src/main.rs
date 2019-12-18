use std::collections::HashMap;
use std::collections::HashSet;

type Grid = HashMap<i32, Scaffolding>;

#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Scaffolding {
    // The ID == y*grid_width + x.  This is easily reversible, so no need to
    // store coordinates separately.
    id: i32,
    // References to the scaffolding's neighbors.  Negative values mean there's
    // no scaffolding in that direction.
    neighbors: HashMap<Dir, i32>,
}

fn flip_dir(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::South,
        Dir::South => Dir::North,
        Dir::East => Dir::West,
        Dir::West => Dir::East,
    }
}

// Calculate the ID of start's neighbor in direction dir.
fn neighbor_id(start: i32, width: i32, dir: Dir) -> i32 {
    match dir {
        Dir::North => start - width,
        Dir::South => start + width,
        Dir::East => start - 1,
        Dir::West => start + 1,
    }
}

// See if start has a neighbor in direction dir.  If so, update the two
// neighbors to reference each other.
fn connect_neighbors(start: i32, width: i32, dir: Dir, grid: &mut Grid, intersections: &mut HashSet<i32>) {
    let n_id = neighbor_id(start, width, dir);
    if start == n_id {
        return;
    }
    if !grid.contains_key(&start) || !grid.contains_key(&n_id) {
        return;
    }

    if let Some(s) = grid.get_mut(&start) {
        s.neighbors.insert(dir, n_id);
    }
    if let Some(n) = grid.get_mut(&n_id) {
        n.neighbors.insert(flip_dir(dir), start);
        if n.neighbors.len() > 2 {
            intersections.insert(n.id);
        }
    }
}


fn build_grid(robo: &mut intcode::IntcodeComp, grid: &mut Grid, intersections: &mut HashSet<i32>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut width: i32 = 0;
    robo.start();
    while let Some(output) = robo.pop_output() {
        let char_val = (output as u8) as char;
        match char_val {
            '\n' => {
                // Newline, means we need to start a new row.
                y += 1;
                if width == 0 {
                    // We now know the total width of the grid.
                    width = x;
                }
                x = 0;
                continue;
            },
            '.' => {
                // Period, means we've scanned an empty space.
                x += 1;
                continue;
            },
            'X' => {
                // 'X', means we've scanned the robot hurtling through empty space.
                // Treat like a regular empty space for now.
                x += 1;
                continue;
            },
            // Below symbols are the robot on scaffolding.  Treat the same as a
            // scaffolding symbol.
            '^' => (),
            'v' => (),
            '<' => (),
            '>' => (),
            // Scaffolding symbol.
            '#' => (),
            _ => { return (width, y); },
        }

        // Add scaffolding to our grid.  Until we get our first newline, we
        // won't know the grid width.  However, that doesn't matter since our
        // ID equation doesn't need to know the width until after we're past
        // the first row anyway.
        let id = y*width + x;
        let space = Scaffolding{
            id: id,
            neighbors: HashMap::new(),
        };
        grid.insert(id, space);
        connect_neighbors(id, width, Dir::North, grid, intersections);
        connect_neighbors(id, width, Dir::South, grid, intersections);
        connect_neighbors(id, width, Dir::East, grid, intersections);
        connect_neighbors(id, width, Dir::West, grid, intersections);

        x += 1;
    }

    return (width, y);
}

fn find_proposals(directions: &str) -> Vec<(String, u8)> {
    let mut proposals: HashMap<String, u8> = HashMap::new();
    for length in 1..21 {
        for start in 0..directions.len()-(length-1) {
            let slice = &directions[start..start+length];
            if !(slice.starts_with("R") || slice.starts_with("L")) || !slice.ends_with(char::is_numeric) {
                continue;
            }
            if slice.contains("A") || slice.contains("B") || slice.contains("C") {
                continue;
            }
            let count: Vec<&str> = directions.matches(slice).collect();
            let efficiency = (slice.len() * count.len()) - count.len();
            proposals.insert(slice.to_string().clone(), efficiency as u8);
        }
    }
    let mut prop_vec: Vec<(String, u8)> = proposals.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
    prop_vec.sort_by(|a, b| a.1.cmp(&b.1));
    return prop_vec;
}

fn main() {
    let line = std::fs::read_to_string("day-seventeen/input.txt").expect("file not found");
    let directions = "R,10,L,12,R,6,R,10,L,12,R,6,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,R,6";
    let main_routine = "C,C,A,B,A,B,A,B,A,C";
    let routine_a = "R,6,R,10,R,12,R,6";
    let routine_b = "R,10,L,12,L,12";
    let routine_c = "R,10,L,12,R,6";
    let tape: Vec<i64> = intcode::to_tape(&line);
    let mut robo = intcode::IntcodeComp::new(tape);
    let mut grid: Grid = HashMap::new();
    let mut intersections: HashSet<i32> = HashSet::new();

    let (width, height) = build_grid(&mut robo, &mut grid, &mut intersections);

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let id = y*width + x;
            if intersections.contains(&id) {
                sum += x*y;
                continue;
            }
        }
    }
    println!("Sum of alignment parameters: {}", sum);

    let _proposals = find_proposals(&directions);

    // for (routine, efficiency) in proposals {
    //     println!("{} : {}", routine, efficiency);
    // }

    for input in vec![main_routine, routine_a, routine_b, routine_c, "n"] {
        print!("robo output: ");
        while let Some(output) = robo.pop_output() {
            let char_val = (output as u8) as char;
            print!("{}", char_val);
        }
        for b in input.as_bytes() {
            print!("{}", *b as char);
            robo.push_input(*b as i64);
        }
        print!("{}", 10 as char);
        robo.push_input(10);
        robo.start();
    }
    println!("robo output: ");
    while let Some(output) = robo.pop_output() {
        println!("{}", output);
    }
}
