extern crate num;
extern crate regex;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

fn parse_to_coords(line: &str) -> (i32, i32, i32) {
    let axis_re = Regex::new("[<>xyz,=]").unwrap();
    let reduced = axis_re.replace_all(line, "");
    let vals: Vec<&str> = reduced.trim().split(" ").collect();
    let x = vals[0].parse::<i32>().unwrap();
    let y = vals[1].parse::<i32>().unwrap();
    let z = vals[2].parse::<i32>().unwrap();

    return (x, y, z);
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len()-1 {
        for j in i+1..moons.len() {
            let moon1 = &moons[i];
            let moon2 = &moons[j];
            // We aggregate the updates first, and apply them at the end.
            let mut update1: (i32, i32, i32) = (0, 0, 0);
            let mut update2: (i32, i32, i32) = (0, 0, 0);
            if moon1.x < moon2.x {
                update1.0 += 1;
                update2.0 -= 1;
            }
            if moon1.x > moon2.x {
                update1.0 -= 1;
                update2.0 += 1;
            }
            if moon1.y < moon2.y {
                update1.1 += 1;
                update2.1 -= 1;
            }
            if moon1.y > moon2.y {
                update1.1 -= 1;
                update2.1 += 1;
            }
            if moon1.z < moon2.z {
                update1.2 += 1;
                update2.2 -= 1;
            }
            if moon1.z > moon2.z {
                update1.2 -= 1;
                update2.2 += 1;
            }
            // By applying them here, we don't need to worry about trying to
            // take multiple mutable references from the vector at once.
            let mut moon = &mut moons[i];
            moon.vx += update1.0;
            moon.vy += update1.1;
            moon.vz += update1.2;
            let mut moon = &mut moons[j];
            moon.vx += update2.0;
            moon.vy += update2.1;
            moon.vz += update2.2;
        }
    }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
    for moon in moons.iter_mut() {
        moon.x += moon.vx;
        moon.y += moon.vy;
        moon.z += moon.vz;
    }
}

fn main() {
    let input = std::fs::read_to_string("day-twelve/input.txt").expect("file not found");
    let lines: Vec<&str> = input.split('\n').collect();
    let mut moons: Vec<Moon> = Vec::with_capacity(4);

    for i in 0..4 {
        let line = lines[i];
        let coords = parse_to_coords(&line);
        let moon = Moon{
            x: coords.0,
            y: coords.1,
            z: coords.2,
            vx: 0,
            vy: 0,
            vz: 0,
        };
        moons.insert(i, moon);
    }

    let checkpoint = 1000;
    let mut steps: u64 = 0;
    let mut x_set: HashSet<((i32, i32), (i32, i32), (i32, i32), (i32, i32))> = HashSet::new();
    let mut y_set: HashSet<((i32, i32), (i32, i32), (i32, i32), (i32, i32))> = HashSet::new();
    let mut z_set: HashSet<((i32, i32), (i32, i32), (i32, i32), (i32, i32))> = HashSet::new();
    let mut x_period = 0;
    let mut y_period = 0;
    let mut z_period = 0;
    loop {
        // Update history sets.
        if x_period == 0 && !x_set.insert(((moons[0].x, moons[0].vx),
                                           (moons[1].x, moons[1].vx),
                                           (moons[2].x, moons[2].vx),
                                           (moons[3].x, moons[3].vx))) {
            x_period = steps;
        }
        if y_period == 0 && !y_set.insert(((moons[0].y, moons[0].vy),
                                           (moons[1].y, moons[1].vy),
                                           (moons[2].y, moons[2].vy),
                                           (moons[3].y, moons[3].vy))) {
            y_period = steps;
        }
        if z_period == 0 && !z_set.insert(((moons[0].z, moons[0].vz),
                                           (moons[1].z, moons[1].vz),
                                           (moons[2].z, moons[2].vz),
                                           (moons[3].z, moons[3].vz))) {
            z_period = steps;
        }
        if x_period != 0 && y_period != 0 && z_period != 0 {
            break;
        }

        // Apply gravity.
        apply_gravity(&mut moons);

        // Apply velocity.
        apply_velocity(&mut moons);

        // Print checkpoint.
        if steps == checkpoint {
            let mut total_energy = 0;
            for moon in moons.iter_mut() {
                let pot_energy = moon.x.abs() + moon.y.abs() + moon.z.abs();
                let kin_energy = moon.vx.abs() + moon.vy.abs() + moon.vz.abs();
                total_energy += pot_energy * kin_energy;
            }

            println!("Moon states after {} steps...", steps);
            for moon in &moons {
                println!("{:?}", moon);
            }
            println!("Total energy: {}", total_energy);
        }
        steps += 1;
    }

    // Find least common multiple between the periods of the three dimensions.
    // That should be the overall period of the cycle across all dimensions.
    println!("periods: {:?}", (x_period, y_period, z_period));
    let mut lcm = lcm(x_period, lcm(y_period, z_period));
    println!("lcm of the periods: {}", lcm);
}
