use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

// Calculates where two line segments intersect.  Implements closed form solution at
// http://www.cs.swan.ac.uk/~cssimon/line_intersection.html.
fn intersects(s1: &Segment, s2: &Segment) -> Option<Point> {
    let x1 = s1.p1.x as f32;
    let x2 = s1.p2.x as f32;
    let x3 = s2.p1.x as f32;
    let x4 = s2.p2.x as f32;
    let y1 = s1.p1.y as f32;
    let y2 = s1.p2.y as f32;
    let y3 = s2.p1.y as f32;
    let y4 = s2.p2.y as f32;

    let num = (y3 - y4)*(x1 - x3) + (x4 - x3)*(y1 - y3);
    let den = (x4 - x3)*(y1 - y2) - (x1 - x2)*(y4 - y3);

    if den == 0.0 {
        return None;
    }
    let t = num / den;

    if t < 0.0 || t > 1.0 {
        None
    } else {
        Some(Point {
            x: (x1 + t * (x2 - x1)) as i32,
            y: (y1 + t * (y2 - y1)) as i32,
        })
    }
}

fn pop_char(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn dist(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

fn closest(p1: Point, p2: Point) -> Point {
    let dist1 = dist(&p1);
    let dist2 = dist(&p2);
    if dist1 < dist2 {
        p1
    } else {
        p2
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");

    let f = File::open(&args[1]).expect("file not found");
    let mut reader = BufReader::new(&f);

    let mut pos = Point{
        x: 0,
        y: 0,
    };
    let to_segment = |&string| {
        let (dir, dist) = pop_char(string);
        let old_pos = pos.clone();
        match dir {
            "U" => pos.y += dist.trim().parse::<i32>().unwrap(),
            "D" => pos.y -= dist.trim().parse::<i32>().unwrap(),
            "L" => pos.x -= dist.trim().parse::<i32>().unwrap(),
            "R" => pos.x += dist.trim().parse::<i32>().unwrap(),
            _ => {},
        }
        Segment{
            p1: old_pos,
            p2: pos.clone(),
        }
    };

    // First wire
    let mut wire = String::new();
    reader.read_line(&mut wire);
    let pieces: Vec<&str> = wire.split(",").collect();
    let wire1_segments: Vec<Segment> = pieces.iter().map(to_segment).collect();
    
    let mut pos = Point{
        x: 0,
        y: 0,
    };
    let to_segment = |&string| {
        let (dir, dist) = pop_char(string);
        let old_pos = pos.clone();
        match dir {
            "U" => pos.y += dist.trim().parse::<i32>().unwrap(),
            "D" => pos.y -= dist.trim().parse::<i32>().unwrap(),
            "L" => pos.x -= dist.trim().parse::<i32>().unwrap(),
            "R" => pos.x += dist.trim().parse::<i32>().unwrap(),
            _ => {},
        }
        Segment{
            p1: old_pos,
            p2: pos.clone(),
        }
    };

    // Second wire
    let mut wire = String::new();
    reader.read_line(&mut wire);
    let pieces: Vec<&str> = wire.split(",").collect();
    let wire2_segments: Vec<Segment> = pieces.iter().map(to_segment).collect();

    // Find intersections
    let mut intersection = Point{ x: 0, y: 0};
    for seg1 in &wire1_segments {
        for seg2 in &wire2_segments {
            match intersects(&seg1, &seg2) {
                Some(p) => {
                    if intersection.x == 0 && intersection.y == 0 {
                        intersection = p;
                    } else {
                        intersection = closest(intersection, p);
                    }
                },
                None => {},
            }
        }
    }

    println!("Closest intersection ({}, {}) has a distance of {}.", intersection.x, intersection.y, dist(&intersection));
}