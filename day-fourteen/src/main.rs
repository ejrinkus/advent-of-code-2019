extern crate regex;

use math::round;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug)]
struct Reaction {
    quantity: u64,
    output: String,
    components: Vec<(u64, String)>,
}

fn parse_line(line: &str) -> Reaction {
    let equals_re = Regex::new(" => ").unwrap();
    let delim_re = Regex::new(", ").unwrap();
    let new_line = equals_re.replace_all(line, "=");
    let new_line = delim_re.replace_all(&new_line, ",");
    
    // Get the reaction inputs.
    let sides: Vec<&str> = new_line.split('=').collect();
    let mut inputs: Vec<(u64, String)> = Vec::new();
    for input in sides[0].split(',') {
        let pieces: Vec<&str> = input.split(' ').collect();
        inputs.push((pieces[0].parse::<u64>().unwrap(), String::from(pieces[1])));
    }
    
    // Get the reaction output.
    let pieces: Vec<&str> = sides[1].split(' ').collect();
    let output = (pieces[0].parse::<u64>().unwrap(), String::from(pieces[1]));

    return Reaction{
        quantity: output.0,
        output: output.1,
        components: inputs,
    }
}

// Returns the list of required materials to make the given chemical at the
// given quantity.  Second return value is surplus quantity.
fn expand_reaction(q: u64, chem: &str, chems: &HashMap<String, Reaction>) -> (VecDeque<(u64, String)>, u64) {
    let next_reaction = chems.get(chem).unwrap();
    let times = round::ceil(q as f64 / next_reaction.quantity as f64, 0) as u64;
    let new_surplus = next_reaction.quantity * times - q;

    let mut total_components: VecDeque<(u64, String)> = VecDeque::new();
    for component in &next_reaction.components {
        let expansion = (component.0 * times, component.1.clone());
        total_components.push_back(expansion); 
    }
    return (total_components, new_surplus);
}

fn make_fuel(quantity: u64, chems: &HashMap<String, Reaction>) -> u64 {
    // Get the reaction that can ultimately make FUEL.
    let mut fuel_reaction = chems.get("FUEL").unwrap().clone();
    for (q, _) in fuel_reaction.components.iter_mut() {
        *q *= quantity;
    }

    // Iteratively expand each component of fuel_reaction into the reactions
    // that make those components, until all the components are ORE.  Any time
    // we expand a chemical, we should keep track of surplus that gets created
    // (since we might be able to use it in other reactions).
    let mut ore_needed: u64 = 0;
    let mut to_expand: VecDeque<(u64, String)> = VecDeque::from(fuel_reaction.components.clone());
    let mut surplus: HashMap<String, u64> = HashMap::new();
    while !to_expand.is_empty() {
        let mut next = to_expand.pop_front().unwrap();
        if next.1 == "ORE" {
            // ORE can't be expanded, so just record it and move on.
            ore_needed += next.0;
            continue;
        }

        // First use any surplus we have for the current chemical.
        let surplus_next = match surplus.get(&next.1) {
            Some(q) => *q,
            None => 0,
        };
        if surplus_next >= next.0 {
            // If we have enough surplus to cover this component, then just
            // pull from the surplus and move on.
            surplus.insert(next.1.clone(), surplus_next - next.0);
            continue;
        }
        next.0 -= surplus_next;

        // After accounting for surplus, figure out how many times we need to
        // run the reaction to make enough of the given chemical (and update
        // surplus accordingly).
        let (expansions, new_surplus) = expand_reaction(next.0, &next.1, &chems);
        surplus.insert(next.1.clone(), new_surplus);

        // Push the expansion back onto our queue for continued expansion.
        for expansion in &expansions {
            to_expand.push_back(expansion.clone()); 
        }
    }
    return ore_needed;
}

fn main() {
    let f = File::open("day-fourteen/input.txt").expect("file not found");
    let reader = BufReader::new(&f);

    // Parse the list of reactions.
    let mut chems: HashMap<String, Reaction> = HashMap::new();
    for line in reader.lines() {
        let reaction = parse_line(&line.unwrap());
        chems.insert(reaction.output.clone(), reaction);
    }

    let mut ore_needed = make_fuel(1, &chems);
    println!("Ore needed per fuel: {}", ore_needed);

    let mut min_fuel = 1 as u64;
    let mut max_fuel = 1000000000 as u64;
    let mut target = 0 as u64;
    let ore_max = 1000000000000 as u64;
    while min_fuel < max_fuel-1 {
        target = (min_fuel + max_fuel) / 2;
        ore_needed = make_fuel(target, &chems);
        // println!("target: {}, ore: {}", target, ore_needed);
        if ore_needed < ore_max {
            min_fuel = target;
        } else if ore_needed > ore_max {
            max_fuel = target;
        } else {
            break;
        }
    }
    println!("Fuel that can be made with 1 trillion ore: {}", target);
}
