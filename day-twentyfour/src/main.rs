use std::collections::HashSet;
use std::collections::VecDeque;

fn print_diversity(grid: u32) {
    for y in 0..5 {
        for x in 0..5 {
            let bit = 1 << (y*5 + x);
            if grid & bit != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn advance(diversity: u32) -> u32 {
    let mut new_diversity = diversity;
    for i in 0..25 {
        let cur_bit = 1 << i;
        let mut adjacent = 0;
        if (i+1) %  5 != 0 && diversity & cur_bit << 1 != 0 { adjacent += 1; }
        if i     %  5 != 0 && diversity & cur_bit >> 1 != 0 { adjacent += 1; }
        if i     < 20      && diversity & cur_bit << 5 != 0 { adjacent += 1; }
        if i     >= 5      && diversity & cur_bit >> 5 != 0 { adjacent += 1; }

        match adjacent {
            0 => new_diversity &= !cur_bit,
            1 => new_diversity |= cur_bit,
            2 => new_diversity ^= cur_bit,
            3 => new_diversity &= !cur_bit,
            4 => new_diversity &= !cur_bit,
            _ => panic!("wat"),
        }
    }
    new_diversity
}

fn part_one(mut diversity: u32) {
    let mut diversities: HashSet<u32> = HashSet::new();
    loop {
        if diversities.contains(&diversity) {
            println!("First repeated biodiversity: {}", diversity);
            break;
        }
        diversities.insert(diversity);
        diversity = advance(diversity);
    }
}

fn advance_all(mut grids: VecDeque<u32>) -> VecDeque<u32> {
    // Push new grids to the top and bottom, since we'll need to be able to to check them.
    // Then clone the deque to serve as the new state of the world.
    grids.push_front(0_u32);
    grids.push_back(0_u32);
    let mut new_grids = grids.clone();

    // Loop over every grid in the deque, and advance each one.
    for d in 0..new_grids.len() {
        let grid = grids.get(d).unwrap();
        let new_grid = new_grids.get_mut(d).unwrap();
        for i in 0..25 {
            if i == 12 { continue; }
            let cur_bit = 1_u32 << i;
            let mut adjacent = 0;

            // Left neighbors.
            if i % 5 != 0 && i != 13 && grid & cur_bit >> 1 != 0 {
                adjacent += 1;
            } else if d > 0 && i % 5 == 0 {
                // This means the left neighbor is in the parent grid.  So we only need to
                // check index 11 in the parent as a neighbor.
                let parent = grids.get(d-1).unwrap();
                let bit = 1_u32 << 11;
                if *parent & bit != 0_u32 { adjacent += 1; }
            } else if d < grids.len()-1 && i == 13 {
                // This means the left neighbor is the child grid.  So we need to check indices
                // [4,9,14,19,24] in the child as neighbors.
                let child = grids.get(d+1).unwrap();
                for shift in [4,9,14,19,24].iter() {
                    let bit = 1_u32 << shift;
                    if *child & bit != 0_u32 { adjacent += 1; }
                }
            }

            // Right neighbors.
            if (i+1) % 5 != 0 && i != 11 && grid & cur_bit << 1 != 0 {
                adjacent += 1;
            } else if d > 0 && (i+1) % 5 == 0 {
                // This means the right neighbor is in the parent grid.  So we only need to
                // check index 13 in the parent as a neighbor.
                let parent = grids.get(d-1).unwrap();
                let bit = 1_u32 << 13;
                if *parent & bit != 0_u32 { adjacent += 1; }
            } else if d < grids.len()-1 && i == 11 {
                // This means the right neighbor is the child grid.  So we need to check indices
                // [0,5,10,15,20] in the child as neighbors.
                let child = grids.get(d+1).unwrap();
                for shift in [0,5,10,15,20].iter() {
                    let bit = 1_u32 << shift;
                    if *child & bit != 0_u32 { adjacent += 1; }
                }
            }

            // Top neighbors.
            if i >= 5 && i != 17 && grid & cur_bit >> 5 != 0 {
                adjacent += 1;
            } else if d > 0 && i < 5 {
                // This means the top neighbor is in the parent grid.  So we only need to
                // check index 7 in the parent as a neighbor.
                let parent = grids.get(d-1).unwrap();
                let bit = 1_u32 << 7;
                if *parent & bit != 0_u32 { adjacent += 1; }
            } else if d < grids.len()-1 && i == 17 {
                // This means the top neighbor is the child grid.  So we need to check indices
                // [20,21,22,23,24] in the child as neighbors.
                let child = grids.get(d+1).unwrap();
                for shift in 20..25 {
                    let bit = 1_u32 << shift;
                    if *child & bit != 0_u32 { adjacent += 1; }
                }
            }

            // Bottom neighbors.
            if i < 20 && i != 7 && grid & cur_bit << 5 != 0 {
                adjacent += 1;
            } else if d > 0 && i >= 20 {
                // This means the bottom neighbor is in the parent grid.  So we only need to
                // check index 17 in the parent as a neighbor.
                let parent = grids.get(d-1).unwrap();
                let bit = 1_u32 << 17;
                if *parent & bit != 0_u32 { adjacent += 1; }
            } else if d < grids.len()-1 && i == 7 {
                // This means the top neighbor is the child grid.  So we need to check indices
                // [0,1,2,3,4] in the child as neighbors.
                let child = grids.get(d+1).unwrap();
                for shift in 0..5 {
                    let bit = 1_u32 << shift;
                    if *child & bit != 0_u32 { adjacent += 1; }
                }
            }

            match adjacent {
                1 => *new_grid |= cur_bit,
                2 => *new_grid ^= cur_bit,
                _ => *new_grid &= !cur_bit,
            }
        }
    }

    new_grids
}

fn part_two(mut grids: VecDeque<u32>) {
    for _ in 0..200 {
        grids = advance_all(grids);
    }

    let mut count = 0;
    for grid in grids {
        for i in 0..25 {
            if grid & 1 << i != 0 {
                count += 1;
            }
        }
    }

    println!("Total bugs after 200 minutes: {}", count);
}

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");

    let start = input.lines()
                      .map(|l| l.as_bytes())
                      .flatten()
                      .rev()
                      .fold(0_u32, |mut acc, c| {
                          acc <<= 1;
                          if *c == '#' as u8 {
                              acc += 1;
                          }
                          acc
                      });
    part_one(start);

    let mut grids: VecDeque<u32> = VecDeque::new();
    grids.push_back(start);
    part_two(grids);
}
