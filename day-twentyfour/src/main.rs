use std::collections::HashSet;

fn get_biodiversity(grid: &Vec<Vec<bool>>) -> u32 {
    grid.iter().flatten().rev().fold(0_u32, |mut acc, val| {
        acc <<= 1;
        if *val {
            acc += 1;
        }
        acc
    })
}

fn next_step(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; 5]; 5];
    for (y, row) in grid.iter().enumerate() {
        for (x, live) in row.iter().enumerate() {
            let mut adjacent = 0;
            if x > 0 && grid[y][x-1] { adjacent += 1; }
            if x < 4 && grid[y][x+1] { adjacent += 1; }
            if y > 0 && grid[y-1][x] { adjacent += 1; }
            if y < 4 && grid[y+1][x] { adjacent += 1; }

            let mut new_live = *live;
            if new_live && adjacent != 1 {
                new_live = false;
            } else if !new_live && (adjacent == 1 || adjacent == 2) {
                new_live = true;
            }
            new_grid[x][y] = new_live;
        }
    }
    new_grid
}

fn next_step2(grid: u32) -> u32 {
    let mut new_grid = grid;
    for i in 0..25 {
        let cur_bit = 1 << i;
        let mut adjacent = 0;
        if (i+1) %  5 != 0 && grid & cur_bit << 1 != 0 { adjacent += 1; }
        if i     %  5 != 0 && grid & cur_bit >> 1 != 0 { adjacent += 1; }
        if i     < 20      && grid & cur_bit << 5 != 0 { adjacent += 1; }
        if i     >= 5      && grid & cur_bit >> 5 != 0 { adjacent += 1; }

        match adjacent {
            0 => new_grid &= !cur_bit,
            1 => new_grid |= cur_bit,
            2 => new_grid ^= cur_bit,
            3 => new_grid &= !cur_bit,
            4 => new_grid &= !cur_bit,
            _ => panic!("wat"),
        }
    }
    new_grid
}

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

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(&path).expect("file not found");

    let mut _grid = vec![vec![false; 5]; 5];
    input.lines()
         .map(|l| l.as_bytes())
         .flatten()
         .enumerate()
         .for_each(|(i, c)| {
            _grid[i/5][i%5] = *c == '#' as u8;
         });

    let mut diversity = input.lines()
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
    println!("Starting biodiversity: {}", diversity);
    print_diversity(diversity);
    println!("");

    let mut diversities: HashSet<u32> = HashSet::new();
    loop {
        if diversities.contains(&diversity) {
            println!("First repeated biodiversity: {}", diversity);
            break;
        }
        diversities.insert(diversity);
        diversity = next_step2(diversity);
    }
    print_diversity(diversity);

    // println!("Final layout:");
    // for row in grid.iter() {
    //     for val in row.iter() {
    //         if *val { print!("#"); }
    //         else { print!("."); }
    //     }
    //     print!("\n");
    // }
}
