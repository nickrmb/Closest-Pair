use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

pub mod deterministic;
pub mod randomized;

#[derive(Clone, Debug)]
pub struct Point(f64, f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point(x, y)
    }
    pub fn euclidean_dist(&self, p: &Point) -> f64 {
        let xdiff = self.0 - p.0;
        let ydiff = self.1 - p.1;
        (xdiff * xdiff + ydiff * ydiff).sqrt()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2},{:.2})", self.0, self.1)
    }
}

// brute force, returns points and euclidean distance
pub fn brute_force(points: &[Rc<Point>]) -> (Rc<Point>, Rc<Point>, f64) {
    let mut p1 = &points[0];
    let mut p2 = &points[1];
    let mut dist = p1.euclidean_dist(&p2);

    for i in 0..(points.len() - 1) {
        let pi = &points[i];
        for j in (i + 1)..points.len() {
            let pj = &points[j];
            let d = pi.euclidean_dist(pj);

            if d < dist {
                p1 = pi;
                p2 = pj;
                dist = d;
            }
        }
    }

    return (Rc::clone(p1), Rc::clone(p2), dist);
}

pub struct Grid {
    grid: HashMap<(usize,usize), Vec<Rc<Point>>>,
    delta: f64,
}

impl Grid {
    pub fn get_coordinates<'a>(&self, x: f64, y: f64) -> (usize, usize) {
        (
            (x / self.delta).floor() as usize,
            (y / self.delta).floor() as usize,
        )
    }

    pub fn new(delta: f64) -> Grid {
        Grid {
            grid: HashMap::new(),
            delta,
        }
    }

    pub fn with_capacity(delta: f64, capacity: usize) -> Grid {
        Grid {
            grid: HashMap::with_capacity(capacity),
            delta,
        }
    }

    pub fn insert(&mut self, point: &Rc<Point>) {
        assert!(point.0 >= 0.0);
        assert!(point.1 >= 0.0);

        let (x, y) = self.get_coordinates(point.0, point.1);

        let cell = self.get_cell_or_create_new(x, y);

        cell.push(Rc::clone(&point));
    }

    pub fn get_cell_or_create_new(&mut self, x: usize, y: usize) -> &mut Vec<Rc<Point>> {
        let map = self.grid.entry((x,y)).or_insert_with(Vec::new);
        map
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> Option<&Vec<Rc<Point>>> {
        self.grid.get(&(x,y))
    }
}
