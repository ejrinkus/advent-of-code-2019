use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coord = (u32, u32);
type Spaces = HashSet<Coord>;
type Keys = HashMap<Coord, u32>;
type Doors = HashMap<Coord, u32>;

// Nested map containing how far a given key is from every other key.
// Outer key is the source key, inner key is the destination key, 
// inner value is a tuple containing (distance between keys, doors on path).
type Edges = HashMap<Coord, HashMap<Coord, (u32, u32)>>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    spaces: Spaces,
    keys: Keys,
    key_field: u32,
    doors: Doors,
    door_field: u32,
    start: Coord,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    coord: Coord,
    keys: u32,
}

// Map a given state with it's current best score.
type StateCache = HashMap<State, u32>;

fn char_to_bit(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        1 << (c as u8 - b'a')
    } else {
        1 << (c as u8 - b'A')
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid{
        spaces: Spaces::new(),
        keys: Keys::new(),
        key_field: 0,
        doors: Doors::new(),
        door_field: 0,
        start: (0, 0),
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                '#' => { continue; },
                '@' => { grid.start = (x as u32, y as u32); },
                c if c.is_ascii_lowercase() => {
                    let bit = char_to_bit(c);
                    grid.key_field |= bit;
                    grid.keys.insert((x as u32, y as u32), bit);
                },
                c if c.is_ascii_uppercase() => {
                    let bit = char_to_bit(c);
                    grid.door_field |= bit;
                    grid.doors.insert((x as u32, y as u32), bit);
                },
                _ => panic!("unexpected input"),
            }
            grid.spaces.insert((x as u32, y as u32));
        }
    }
    grid
}

// Explore finds all the edges (i.e. paths between keys) in the grid.
// It also finds all the paths from the start tile to all keys.
fn explore(grid: &Grid, edges: &mut Edges, start: &Coord) {
    edges.insert(start.clone(), HashMap::new());

    let mut explored: HashSet<Coord> = HashSet::new();
    let mut to_explore: VecDeque<(Coord, u32, u32)> = VecDeque::new();
    to_explore.push_back((start.clone(), 0, 0));

    while !to_explore.is_empty() {
        let (curr, dist, mut doors) = to_explore.pop_front().unwrap();
        if !grid.spaces.contains(&curr) { continue; }
        if explored.contains(&curr) { continue; }
        if (grid.keys.contains_key(&curr) || curr == grid.start) && curr != *start {
            let edge_map = &mut edges.get_mut(&start).unwrap();
            edge_map.insert(curr.clone(), (dist, doors.clone()));
        }
        if grid.doors.contains_key(&curr) && curr != *start {
            doors |= grid.doors.get(&curr).unwrap();
        }

        [ (curr.0-1, curr.1),
          (curr.0+1, curr.1),
          (curr.0, curr.1-1),
          (curr.0, curr.1+1) ].iter().for_each(|c| to_explore.push_back((*c, dist+1, doors.clone())));
        explored.insert(curr);
    }
}

// Reachable returns the set of keys that are reachable from a given point,
// with a given set of already collected keys.
fn reachable(grid: &Grid, edges: &Edges, start: &Coord, keys: u32) -> Vec<(Coord, u32, u32)> {
    let mut reachable: Vec<(Coord, u32, u32)> = Vec::new();
    for (dest, (dist, doors)) in edges.get(start).unwrap() {
        if !grid.keys.contains_key(&dest) { continue; }
        // Skip any keys that are already collected, or that we can't reach due
        // to doors that we don't yet have keys for.
        let key = *grid.keys.get(&dest).unwrap();
        if (key & keys) != 0 { continue; }
        if ((keys ^ doors) & doors) != 0 { continue; }
        reachable.push((*dest, key, *dist));
    }
    return reachable;
}

// Search figures out the shortest path to acquire all keys, given a starting
// tile and current key set.
fn search(grid: &Grid, edges: &Edges, cache: &mut StateCache, state: &State) -> u32 {
    if state.keys & grid.key_field == grid.key_field {
        return 0;
    }
    if cache.contains_key(state) {
        return *cache.get(state).unwrap();
    }
    let mut shortest = std::u32::MAX;
    for (kc, kb, kd) in reachable(grid, edges, &state.coord, state.keys) {
        let next = State{
            coord: kc,
            keys: state.keys | kb,
        };
        let dist = search(grid, edges, cache, &next);
        shortest = std::cmp::min(shortest, dist + kd);
    }
    cache.insert(state.clone(), shortest);
    return shortest;
}

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");

    let grid = parse_input(&input);

    let mut edges = Edges::new();
    grid.keys.keys().chain([grid.start].iter()).for_each(|l| explore(&grid, &mut edges, &l));

    let mut cache = StateCache::new();
    let start_state = State{
        coord: grid.start,
        keys: 0,
    };
    println!("Shortest path: {}", search(&grid, &edges, &mut cache, &start_state));
}
