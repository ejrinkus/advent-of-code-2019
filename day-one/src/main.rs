use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");

    let mut module_fuel = 0;
    let mut total_fuel = 0;
    let f = File::open(&args[1]).expect("file not found");
    let reader = BufReader::new(&f);
    for line in reader.lines() {
        let line = line.unwrap().to_string();
        let val = line.parse::<i32>().unwrap();
        let mut add_fuel = (val / 3) - 2;
        module_fuel += add_fuel;
        while add_fuel > 0 {
            total_fuel += add_fuel;
            add_fuel = (add_fuel / 3) - 2;
        }
    }
    println!("Sum of module fuel requirements: {}", module_fuel);
    println!("Sum of total fuel requirements: {}", total_fuel);
}