use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// struct Body {
//     key: String,
//     satellites: Vec<String>,
// }

// impl Body {
//     pub fn new(k: &str) -> Body {
//         Body{
//             key: k.to_owned(),
//             satellites: Vec::new(),
//         }
//     }

//     pub fn add_satellite(&mut self, satellite: &str) {
//         self.satellites.push(satellite.to_owned());
//     }
// }

fn get_orbits(root: &str, bodies: &HashMap<String, Vec<String>>, depth: u32) -> u32 {
    let mut count = depth;
    for s in bodies.get(root).unwrap() {
        count += get_orbits(s, bodies, depth+1);
    }
    return count;
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

    println!("Total number of orbits: {}", get_orbits("COM", &bodies, 0));
}