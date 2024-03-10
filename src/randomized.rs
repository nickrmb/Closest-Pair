use std::rc::Rc;
use rand::{thread_rng, seq::SliceRandom};

use crate::{Grid, Point};

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn correctness() {
        for _ in 0..100 {
            let mut points = vec![];

            let mut rng = rand::thread_rng();
            for _ in 0..100 {
                points.push(Rc::new(Point::new(
                    rng.gen::<f64>() * 1000.0,
                    rng.gen::<f64>() * 1000.0,
                )));
            }

            let result1 = solve(&mut points);
            let result2 = crate::brute_force(&points);

            assert!(result1.2.eq(&result2.2));
        }
    }
}

pub fn solve(points: &mut Vec<Rc<Point>>) -> (Rc<Point>, Rc<Point>, f64) {
    assert!(points.len() >= 2);

    // random permutation
    points.shuffle(&mut thread_rng());

    // take first two points as starting reference
    let mut p1 = Rc::clone(&points[0]);
    let mut p2 = Rc::clone(&points[1]);
    let mut dist = p1.euclidean_dist(&p2);

    // create new grid with distance as delta
    let mut grid = Grid::new(dist);
    grid.insert(&p1);
    grid.insert(&p2);

    for i in 2..points.len() {
        // incremental step

        let p = &points[i];
        let (x, y) = grid.get_coordinates(p.0, p.1); // get grid coordinates

        // list of neighboring (eligable) cells
        let mut cells_to_check: Vec<(usize, usize)> =
            vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)];
        if x > 0 {
            cells_to_check.push((x - 1, y));
            cells_to_check.push((x - 1, y + 1));
            if y > 0 {
                cells_to_check.push((x - 1, y - 1));
            }
        }
        if y > 0 {
            cells_to_check.push((x, y - 1));
            cells_to_check.push((x + 1, y - 1));
        }

        // truth value whether grid has to be adapted
        let mut changed: bool = false;

        // go through all neighboring cells
        for cell in cells_to_check {
            let points = grid.get_cell(cell.0, cell.1);

            // check if cell is non-empty
            if let Some(points) = points {

                // go through all points in cell
                for point in points {

                    // calc distance to point
                    let d = p.euclidean_dist(&point);

                    // check if its new minimum
                    if d < dist {
                        p1 = Rc::clone(&p);
                        p2 = Rc::clone(&point);
                        dist = d;
                        changed = true;
                    }
                }
            }
        }

        if changed {
            // create new grid
            grid = Grid::new(dist);

            // insert all previous points
            for j in 0..i {
                grid.insert(&points[j]);
            }
        }

        // insert current point
        grid.insert(p);
    }

    (p1, p2, dist)
}
