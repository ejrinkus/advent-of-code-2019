use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_quadrant(slope: &(i8, i8)) -> u8 {
    if slope.0 >= 0 && slope.1 < 0 {
        // Upper-right quad.
        return 1;
    } else if slope.0 >= 0 && slope.1 >= 0 {
        // Lower-right quad.
        return 2;
    } else if slope.0 < 0 && slope.1 >= 0 {
        // Lower-left quad.
        return 3;
    } else {
        // Upper-left quad.
        return 4;
    }
}

fn clockwise_ordering(left: &(i8, i8), right: &(i8, i8)) -> Ordering {
    let quad_left = get_quadrant(left);
    let quad_right = get_quadrant(right);
    if quad_left < quad_right {
        return Ordering::Less;
    }
    if quad_left > quad_right {
        return Ordering::Greater;
    }

    // Same quadrant.
    let mut left_ratio = left.1 as f64 / left.0 as f64;
    let mut right_ratio = right.1 as f64 / right.0 as f64;
    // if left_ratio == std::f64::NEG_INFINITY {
    //     left_ratio = std::f64::INFINITY;
    // }
    // if right_ratio == std::f64::NEG_INFINITY {
    //     right_ratio = std::f64::INFINITY;
    // }

    if left_ratio == right_ratio {
        return Ordering::Equal;
    }
    if left_ratio < right_ratio {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

fn slope_to_asteroid(start: &(u8, u8), slope: &(i8, i8), asteroids: &HashMap<(u8, u8), HashSet<(i8, i8)>>) -> (u8, u8) {
    let mut start_cpy = start.clone();
    loop {
        start_cpy.0 = (slope.0 + start_cpy.0 as i8) as u8;
        start_cpy.1 = (slope.1 + start_cpy.1 as i8) as u8;
        if asteroids.contains_key(&start_cpy) {
            return start_cpy;
        }
    }
}

fn get_slope(start: &(u8, u8), end: &(u8, u8)) -> (i8, i8) {
    if start.0 == end.0 {
        if start.1 < end.1 {
            // Start is to the left of end on the same x-value, so reduce this to (0, 1).
            return (0, 1);
        }
        // Start is to the right of end on the same x-value, so reduce this to (0, -1).
        return (0, -1);
    }
    if start.1 == end.1 {
        if start.0 < end.0 {
            // Start is above end on the same y-value, so reduce this to (1, 0).
            return (1, 0);
        }
        // Start is below end on the same y-value, so reduce this to (-1, 0).
        return (-1, 0);
    }
    let dx = end.0 as i8 - start.0 as i8;
    let dy = end.1 as i8 - start.1 as i8;

    // Get GCD of dx and dy via Euclid's algorithm.  Once the loop is done, den is the GCD
    let mut num = max(dx.abs(), dy.abs());
    let mut den = min(dx.abs(), dy.abs());
    let mut rem = num % den;
    while rem != 0 {
        num = den;
        den = rem;
        rem = num % den;
    }

    return (dx / den, dy / den);
}

fn add_and_check(coords: (u8, u8), asteroids: &mut HashMap<(u8, u8), HashSet<(i8, i8)>>) {
    let mut slopes: HashSet<(i8, i8)> = HashSet::new();
    for (key, val) in asteroids.iter_mut() {
        let slope = get_slope(&coords, key);
        let slope_inv = (slope.0 * -1, slope.1 * -1);
        val.insert(slope_inv);
        slopes.insert(slope);
    }
    asteroids.insert(coords, slopes);
}

fn main() {
    let f = File::open("day-ten/input.txt").expect("file not found");
    let reader = BufReader::new(&f);
    let lines = reader.lines();

    // Map each coordinate to a list of slopes.  Every slope represents a line
    // upon which at least one asteroid lies.  The number of such slopes is the
    // number of asteroids visible to the key asteroid.
    let mut asteroids: HashMap<(u8, u8), HashSet<(i8, i8)>> = HashMap::new();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.unwrap().chars() {
            if c == '#' {
                add_and_check((x, y), &mut asteroids);
            }
            x += 1;
        }
        y += 1;
    }

    let mut best_loc = (0, 0);
    let mut best_set = HashSet::new();
    for (loc, set) in &asteroids {
        if set.len() > best_set.len() {
            best_set = set.clone();
            best_loc = loc.clone();
        }
    }

    // Add one to best_set since the asteroid can see itself.
    println!("The best choice is {:?}, it can see {} asteroids", best_loc, best_set.len());

    // Convert hashset to sorted list.  Ordering is based on slope.  The list
    // should start with asteroids directly above 
    let mut visible_set = best_set.clone();
    let mut counter = 0;
    while asteroids.len() > 1 {
        let mut visible: Vec<(i8, i8)> = visible_set.iter().map(|a| a.clone()).collect();
        visible.sort_by(|a, b| clockwise_ordering(a, b));
        for pair in &visible {
            counter += 1;
            let target = slope_to_asteroid(&best_loc, &pair, &asteroids);
            if counter == 200 {
                println!("{}: {:?}", counter, target);
            }
            asteroids.remove(&target);
        }
        visible_set = HashSet::new();
        for (key, val) in asteroids.iter_mut() {
            let slope = get_slope(&best_loc, key);
            visible_set.insert(slope);
        }
    }
}
