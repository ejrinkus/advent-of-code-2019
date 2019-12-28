use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coord = (u32, u32);
type Spaces = HashSet<Coord>;
type Portals = HashMap<Coord, Coord>;

struct Grid {
    spaces: Spaces,
    portals: Portals,
    start: Coord,
    end: Coord,
}

fn parse_input(input: &str) -> Grid {
    let mut symbol_grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        symbol_grid.push(row);
    }

    let mut grid = Grid{
        spaces: Spaces::new(),
        portals: Portals::new(),
        start: (0, 0),
        end: (0, 0),
    };
    let mut label_cache: HashMap<String, Coord> = HashMap::new();
    for (y, row) in symbol_grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' { continue; }
            grid.spaces.insert((x as u32, y as u32));

            // List of coord pairs, where each pair represents the possible
            // locations of a portal label.
            let x_u32 = x as u32;
            let y_u32 = y as u32;
            let mut maybe_labels: Vec<(Coord, Coord)> = Vec::new();

            // Check for a portal label to the left.
            if x > 1 {
                maybe_labels.push(((x_u32-2, y_u32), (x_u32-1, y_u32)));
            }
            // Check for a portal label above.
            if y > 1 {
                maybe_labels.push(((x_u32, y_u32-2), (x_u32, y_u32-1)));
            }
            // Check for a portal label below.
            if y < symbol_grid.len() - 2 {
                maybe_labels.push(((x_u32, y_u32+1), (x_u32, y_u32+2)));
            }
            // Check for a portal label to the right.
            if x < row.len() - 2 {
                maybe_labels.push(((x_u32+1, y_u32), (x_u32+2, y_u32)));
            }
            for (lc1, lc2) in maybe_labels {
                let char1 = symbol_grid[lc1.1 as usize][lc1.0 as usize];
                let char2 = symbol_grid[lc2.1 as usize][lc2.0 as usize];
                if !char1.is_ascii_uppercase() || !char2.is_ascii_uppercase() { continue; }
                let label = String::from_utf8(vec![char1 as u8, char2 as u8]).unwrap();

                if label == "AA" {
                    grid.start = (x_u32, y_u32);
                } else if label == "ZZ" {
                    grid.end = (x_u32, y_u32);
                } else if label_cache.contains_key(&label) {
                    let coord1 = label_cache.remove(&label).unwrap();
                    let coord2 = (x_u32, y_u32);
                    grid.portals.insert(coord1, coord2);
                    grid.portals.insert(coord2, coord1);
                } else {
                    label_cache.insert(label, (x_u32, y_u32));
                }
            }
        }
    }
    grid
}

fn search(grid: &Grid) -> u32 {
    let mut to_explore: VecDeque<(Coord, u32)> = VecDeque::new();
    to_explore.push_back((grid.start, 0));

    let mut explored: HashSet<Coord> = HashSet::new();
    while !to_explore.is_empty() {
        let (curr, dist) = to_explore.pop_front().unwrap();
        if curr == grid.end {
            return dist;
        }
        if !grid.spaces.contains(&curr) { continue; }
        if explored.contains(&curr) { continue; }

        // Left
        if curr.0 > 0 {
            to_explore.push_back(((curr.0-1, curr.1), dist+1));
        }
        // Up
        if curr.1 > 0 {
            to_explore.push_back(((curr.0, curr.1-1), dist+1));
        }
        // Right
        to_explore.push_back(((curr.0+1, curr.1), dist+1));
        // Down
        to_explore.push_back(((curr.0, curr.1+1), dist+1));
        // Portal
        if grid.portals.contains_key(&curr) {
            let portal = grid.portals.get(&curr).unwrap();
            to_explore.push_back((portal.clone(), dist+1));
        }

        explored.insert(curr);
    }
    0
}

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");

    let grid = parse_input(&input);

    let dist = search(&grid);
    println!("shortest path: {}", dist);
}
