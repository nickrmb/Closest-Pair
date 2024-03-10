use closest_pair::*;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use std::time::Instant;

fn create_random_points(n: usize) -> Vec<Rc<Point>> {
    let mut points: Vec<Rc<Point>> = Vec::with_capacity(n);

    let mut rnd = thread_rng();

    for _ in 0..n {
        points.push(Rc::new(Point::new(
            rnd.gen::<f64>() * 1000.0,
            rnd.gen::<f64>() * 1000.0,
        )));
    }

    points
}

fn main() {
    println!("len,brute_force,deterministic,randomized");
    for i in 2..=400 {
        let n = i * i;
        let mut points = create_random_points(n);

        let t1 = Instant::now();
        let _ = brute_force(&points);
        let t2 = Instant::now();
        let _ = deterministic::solve(&mut points);
        let t3 = Instant::now();
        let _ = randomized::solve(&mut points);
        let t4 = Instant::now();

        let t1 = t2.duration_since(t1);
        let t2 = t3.duration_since(t2);
        let t3 = t4.duration_since(t3);

        println!("{n},{},{},{}", t1.as_micros(), t2.as_micros(), t3.as_micros());
    }
}
