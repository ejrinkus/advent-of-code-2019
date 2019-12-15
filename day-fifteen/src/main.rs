use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PathResult {
    Invalid,
    Valid,
    Oxygen((i32, i32)),
}

fn is_path_valid(path: &Vec<u8>, robo: &mut intcode::IntcodeComp, map: &mut HashSet<(i32, i32)>) -> PathResult {
    let mut backtrack: VecDeque<u8> = VecDeque::new();
    let mut path_result = PathResult::Valid;
    let mut pos: (i32, i32) = (0, 0);
    for m in path {
        robo.push_input(*m as i64);
        robo.start();
        let result = robo.pop_output().unwrap();
        if result == 0 {
            path_result = PathResult::Invalid;
            break;
        }
        match m {
            1 => {
                pos.1 += 1;
                backtrack.push_front(2);
            },
            2 => {
                pos.1 -= 1;
                backtrack.push_front(1);
            },
            3 => {
                pos.0 -= 1;
                backtrack.push_front(4);
            },
            4 => {
                pos.0 += 1;
                backtrack.push_front(3);
            },
            _ => panic!("unknown direction"),
        }
        match result {
            1 => path_result = PathResult::Valid,
            2 => path_result = PathResult::Oxygen(pos.clone()),
            _ => panic!("unknown path result"),
        }
    }
    // try adding pos to the set. If it's already there, then trim this
    // path by marking it invalid.
    let success = map.insert(pos.clone());
    if !success {
        path_result = PathResult::Invalid;
    }
    // unwind back to the start position
    for m in backtrack {
        robo.push_input(m as i64);
        robo.start();
        let result = robo.pop_output().unwrap();
        if result != 1 {
            panic!("unexpected result while backtracking");
        }
    }
    return path_result;
}

fn main() {
    let line = std::fs::read_to_string("day-fifteen/input.txt").expect("file not found");
    let tape: Vec<i64> = intcode::to_tape(&line);
    let mut robo = intcode::IntcodeComp::new(tape);
    let mut paths: VecDeque<Vec<u8>> = VecDeque::new();
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut oxygen = (0, 0);
    paths.push_back(vec![1]);
    paths.push_back(vec![2]);
    paths.push_back(vec![3]);
    paths.push_back(vec![4]);
    map.insert((0, 0));
    robo.start();
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        let path_result = is_path_valid(&path, &mut robo, &mut map);
        if path_result == PathResult::Invalid { continue; }

        // If the path is valid, then push the path back onto the queue 4
        // times, and for each clone add an extra direction to the path.
        let last_move = path.last().unwrap();
        let skip_move = match last_move {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("unknown direction"),
        };
        for i in 1..5 {
            if i == skip_move { continue; }
            let mut new_path = path.clone();
            new_path.push(i);
            paths.push_back(new_path);
        }

        if let PathResult::Oxygen(pos) = path_result {
            println!("Shortest path: {}", path.len());
            oxygen = pos.clone();
        }
    }

    println!("oxygen: {:?}", oxygen);
    let mut minutes = 0;
    let mut breathable: HashSet<(i32, i32)> = HashSet::new();
    breathable.insert(oxygen);
    while !map.is_empty() {
        let mut next_breathable: HashSet<(i32, i32)> = HashSet::new();
        for space in breathable {

            if map.remove(&(space.0-1, space.1)) {
                next_breathable.insert((space.0-1, space.1));
            }
            if map.remove(&(space.0+1, space.1)) {
                next_breathable.insert((space.0+1, space.1));
            }
            if map.remove(&(space.0, space.1-1)) {
                next_breathable.insert((space.0, space.1-1));
            }
            if map.remove(&(space.0, space.1+1)) {
                next_breathable.insert((space.0, space.1+1));
            }
        }
        breathable = next_breathable;
        minutes += 1;
    }
    println!("minutes: {}", minutes);
}
