use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// First return value is the total number of orbits (direct and indirect).
// Second return value is the depth of the first planet orbited by both Santa (SAN) and you (YOU).
// Third return value is the depth of the planet orbited by Santa (SAN).
// Fourth return value is the depth of the planet orbited by you (YOU).
fn trace_orbits(root: &str, bodies: &HashMap<String, Vec<String>>, depth: u32) -> (u32, u32, u32, u32) {
    let mut count = depth;
    let mut common = 0;
    let mut san = 0;
    let mut you = 0;
    for s in bodies.get(root).unwrap() {
        if s == "SAN" {
            san = depth;
            continue;
        }
        if s == "YOU" {
            you = depth;
            continue;
        }

        let (new_count, new_common, new_san, new_you) = trace_orbits(s, bodies, depth+1);
        count += new_count;
        if common == 0 {
            common = new_common;
        }
        if san == 0 {
            san = new_san;
        }
        if you == 0 {
            you = new_you;
        }
    }

    if common == 0 && san != 0 && you != 0 {
        common = depth;
    }
    return (count, common, san, you);
}

fn main() {
    let f = File::open("day-six/input.txt").expect("file not found");
    let reader = BufReader::new(&f);
    let lines = reader.lines();

    // Build the tree of orbits.
    let mut bodies: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let safe_line = line.unwrap();
        let pieces: Vec<&str> = safe_line.split(")").collect();

        let satellites = bodies.entry(pieces[0].to_owned()).or_insert(Vec::new());
        satellites.push(pieces[1].to_owned());

        bodies.entry(pieces[1].to_owned()).or_insert(Vec::new());
    }

    let (count, common, san, you) = trace_orbits("COM", &bodies, 0);
    println!("Total number of orbits: {}", count);
    println!("Santa is {} jumps from COM, and you are {} jumps from COM.", san, you);
    println!("Your nearest shared orbit is {} jumps from COM.", common);
    println!("There are {} jumps between you and Santa.", (san - common) + (you - common));
}