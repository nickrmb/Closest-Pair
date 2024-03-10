use std::{fs, rc::Rc};
use std::time::Instant;

use closest_pair::*;

use crate::{deterministic, randomized};

fn main() {
    let mut args = std::env::args().skip(1);
    let algorithm = args
        .next()
        .expect("FALSE USAGE: <algorithm> <point1> <point2> <point3> ...\n         OR: <algorithm> <file path>");
    let astr = algorithm.to_lowercase();
    if astr != "brute_force" && astr != "randomized" && astr != "deterministic" {
        eprintln!("Algorithm {algorithm} not found.");
        eprintln!("Available: brute_force, deterministic and randomized");
        return;
    }

    let mut arg: Vec<String> = args.collect();

    if arg.len() == 0 {
        eprintln!("FALSE USAGE: <algorithm> <point1> <point2> <point3> ...\n         OR: <algorithm> <file path>");
        return;
    }

    if arg.len() == 1 {
        arg = fs::read_to_string(&arg[0])
            .expect("File not Found!")
            .split("\n").map(|a| String::from(a)).collect();
    }


    let mut points: Vec<Rc<Point>> = Vec::new();

    for (i, point_str) in arg.iter().enumerate() {
        if point_str.as_str() == "" {
            continue;
        }

        let split = point_str.split(",");
        let nums: Vec<&str> = split.collect();
        if nums.len() != 2 {
            eprintln!("Points are not 2-dimensional, numbers must be comma separated, e.g. 3.8,-4");
            eprintln!("{}-th point was given as: {}", i+1, point_str);
            return;
        }

        let x: f64 = nums[0].parse().expect(
            format!(
                "First argument of {}-th point could not be converted to f64: {}",
                i+1, nums[0]
            )
            .as_str(),
        );

        let y: f64 = nums[1].parse().expect(
            format!(
                "Second argument of {}-th point could not be converted to f64: {}",
                i+1, nums[1]
            )
            .as_str(),
        );

        points.push(Rc::new(Point::new(x,y)));
    }

    let result: (Rc<Point>, Rc<Point>, f64);

    println!("Running {astr} algorithm");

    let t1 = Instant::now();

    if astr == "brute_force" {
        result = brute_force(&points);
    } else if astr == "deterministic" {
        result = deterministic::solve(&mut points);
    } else {
        result = randomized::solve(&mut points);
    }

    let t2 = Instant::now();

    println!("Finished in {} ms", t2.duration_since(t1).as_millis());
    println!("Point 1: {}", result.0);
    println!("Point 2: {}", result.1);
    println!("distance: {}", result.2);

}
