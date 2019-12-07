use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Segment {
    p1: Point,
    p2: Point,
}

fn intersects(s1: &Segment, s2: &Segment) -> Option<Point> {
    if s1.p1.x == s1.p2.x {
        // s1 is vertical
        let x1 = s1.p1.x;
        if s2.p1.x == s2.p2.x {
            // s2 is also vertical, so they can't be intersecting
            return None;
        }
        let y2 = s2.p1.y;
        if x1 <= std::cmp::max(s2.p1.x, s2.p2.x) && x1 >= std::cmp::min(s2.p1.x, s2.p2.x)
            && y2 <= std::cmp::max(s1.p1.y, s1.p2.y) && y2 >= std::cmp::min(s1.p1.y, s1.p2.y) {
            // If the x-value of the vertical line is between the x-values of the horizontal line,
            // AND the y-value of the horizontal line is between the y-values of the vertical line,
            // then we know the lines intersect.
            return Some(Point{x: x1, y: y2});
        }
    } else {
        // s1 is horizontal
        let y1 = s1.p1.y;
        if s2.p1.y == s2.p2.y {
            // s2 is also horizontal, so they can't be intersecting
            return None;
        }
        let x2 = s2.p1.x;
        if x2 <= std::cmp::max(s1.p1.x, s1.p2.x) && x2 >= std::cmp::min(s1.p1.x, s1.p2.x)
            && y1 <= std::cmp::max(s2.p1.y, s2.p2.y) && y1 >= std::cmp::min(s2.p1.y, s2.p2.y) {
            // If the x-value of the vertical line is between the x-values of the horizontal line,
            // AND the y-value of the horizontal line is between the y-values of the vertical line,
            // then we know the lines intersect.
            return Some(Point{x: x2, y: y1});
        }
    }
    None
}

// fn dist(p: &Point) -> i32 {
//     p.x.abs() + p.y.abs()
// }

// fn closest(p1: Point, p2: Point) -> Point {
//     let dist1 = dist(&p1);
//     let dist2 = dist(&p2);
//     if dist1 < dist2 {
//         p1
//     } else {
//         p2
//     }
// }

fn pop_char(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn to_segments(turns: Vec<&str>) -> Vec<(Segment, i32)> {
    let mut segments: Vec<(Segment, i32)> = Vec::new();
    let mut pos = Point {
        x: 0,
        y: 0,
    };
    let mut total_dist = 0;

    for turn in turns {
        let (dir, dist_str) = pop_char(turn);
        let old_pos = pos.clone();
        let dist = dist_str.trim().parse::<i32>().unwrap();
        total_dist += dist;
        match dir {
            "U" => pos.y += dist,
            "D" => pos.y -= dist,
            "L" => pos.x -= dist,
            "R" => pos.x += dist,
            _ => {},
        }
        segments.push((Segment{
            p1: old_pos,
            p2: pos.clone(),
        }, total_dist));
    }

    segments
}

fn main() {
    let f = File::open("day-three/input.txt").expect("file not found");
    let mut reader = BufReader::new(&f);

    // First wire
    let mut wire = String::new();
    reader.read_line(&mut wire).expect("input missing first wire");
    let wire1_segments: Vec<(Segment, i32)> = to_segments(wire.split(",").collect());

    // Second wire
    let mut wire = String::new();
    reader.read_line(&mut wire).expect("input missing second wire");
    let wire2_segments: Vec<(Segment, i32)> = to_segments(wire.split(",").collect());

    // Find intersections
    let mut intersection = Point{x: 0, y: 0};
    let mut closest = 0;
    for (seg1, dist1) in &wire1_segments {
        for (seg2, dist2) in &wire2_segments {
            match intersects(&seg1, &seg2) {
                Some(p) => {
                    // For both segments, subtract the distance between the intersection and the second point
                    // of each segment.  This is because dist1 and dist2 include the full segment.
                    let x_diff;
                    let y_diff;
                    if seg1.p1.x == p.x {
                        // seg1 is the vertical.
                        x_diff = seg2.p2.x - p.x;
                        y_diff = seg1.p2.y - p.y;
                    } else {
                        // seg1 is the horizontal.
                        x_diff = seg1.p2.x - p.x;
                        y_diff = seg2.p2.y - p.y;
                    }
                    let new_dist = dist1 + dist2 - x_diff.abs() - y_diff.abs();
                    if p.x != 0 || p.y != 0 {
                        if new_dist < closest || closest == 0 {
                            closest = new_dist;
                            intersection = p;
                            println!("Closest intersection ({}, {}) has a distance of {}.", intersection.x, intersection.y, closest);
                        }
                    }
                },
                None => {},
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects() {
        let cases = vec![
            (
                Segment{
                    p1: Point{x: 3, y: 5},
                    p2: Point{x: 3, y: 2},
                },
                Segment{
                    p1: Point{x: 2, y: 3},
                    p2: Point{x: 6, y: 3},
                },
                Some(Point{x: 3, y: 3})
            ),
            (
                Segment{
                    p1: Point{x: -3, y: 5},
                    p2: Point{x: -3, y: 2},
                },
                Segment{
                    p1: Point{x: -3, y: 2},
                    p2: Point{x: 6, y: 2},
                },
                Some(Point{x: -3, y: 2})
            ),
            (
                Segment{
                    p1: Point{x: 3, y: 5},
                    p2: Point{x: 3, y: 2},
                },
                Segment{
                    p1: Point{x: -3, y: -2},
                    p2: Point{x: 6, y: -2},
                },
                None
            ),
        ];

        for (left, right, expected) in cases {
            assert_eq!(intersects(&left, &right), expected);
        }
    }

    // #[test]
    // fn test_dist() {
    //     let cases = vec![
    //         (Point{x: 43, y: 22}, 65),
    //         (Point{x: -43, y: 22}, 65),
    //         (Point{x: 43, y: -22}, 65),
    //         (Point{x: -43, y: -22}, 65),
    //     ];

    //     for (input, expected) in cases {
    //         assert_eq!(dist(&input), expected);
    //     }
    // }

    // #[test]
    // fn test_closest() {
    //     let cases = vec![
    //         (Point{x: 43, y: 22}, Point{x: 43, y: 22}, Point{x: 43, y: 22}),
    //         (Point{x: 43, y: 22}, Point{x: 43, y: 21}, Point{x: 43, y: 21}),
    //         (Point{x: -43, y: 22}, Point{x: 40, y: 20}, Point{x: 40, y: 20}),
    //         (Point{x: 43, y: -22}, Point{x: 50, y: 30}, Point{x: 43, y: -22}),
    //         (Point{x: -43, y: -22}, Point{x: -20, y: 40}, Point{x: -20, y: 40}),
    //     ];

    //     for (left, right, expected) in cases {
    //         assert_eq!(closest(left, right), expected);
    //     }
    // }

    #[test]
    fn test_pop_char() {
        let cases = vec![
            ("U1", ("U", "1")),
            ("D12", ("D", "12")),
            ("L123", ("L", "123")),
            ("R", ("R", "")),
        ];

        for (input, expected) in cases {
            assert_eq!(pop_char(input), expected);
        }
    }

    #[test]
    fn test_to_segments() {
        let cases = vec![
            (vec!["U1","D12","L123","R1234"], vec![
                (Segment {
                    p1: Point{x: 0, y: 0},
                    p2: Point{x: 0, y: 1},
                }, 1),
                (Segment {
                    p1: Point{x: 0, y: 1},
                    p2: Point{x: 0, y: -11},
                }, 13),
                (Segment {
                    p1: Point{x: 0, y: -11},
                    p2: Point{x: -123, y: -11},
                }, 136),
                (Segment {
                    p1: Point{x: -123, y: -11},
                    p2: Point{x: 1111, y: -11},
                }, 1370),
            ]),
        ];

        for (input, expected) in cases {
            let got: Vec<(Segment, i32)> = to_segments(input);
            for i in 0..got.len() {
                let (got_seg, got_dist) = &got[i];
                let (want_seg, want_dist) = &expected[i];
                assert_eq!(got_seg, want_seg);
                assert_eq!(got_dist, want_dist);
            }
        }
    }
}