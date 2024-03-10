use crate::*;
use std::rc::Rc;

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
            let result2 = brute_force(&points);

            assert!(result1.2.eq(&result2.2));
        }
    }
}

// use deterministic approach, returns points and squared euclidean distance
pub fn solve(points: &mut Vec<Rc<Point>>) -> (Rc<Point>, Rc<Point>, f64) {
    assert!(points.len() >= 2);

    if points.len() <= 5 {
        // brute force if we only have few points
        return brute_force(points);
    }

    // sort by x
    points.sort_unstable_by(|p1, p2| p1.0.total_cmp(&p2.0));

    // apply divide and conquer approach
    divide_and_conquer(points)
}

// divide and conquer approach, returns points and squared euclidean distance
fn divide_and_conquer(points: &mut [Rc<Point>]) -> (Rc<Point>, Rc<Point>, f64) {
    let n = points.len(); // number of elements

    if n <= 5 {
        points.sort_unstable_by(|p1, p2| p1.1.total_cmp(&p2.1));
        return brute_force(points);
    }

    // middle element
    let m = n / 2;

    // median
    let median = if n % 2 == 0 {
        (points[m - 1].0 + points[m].0) / 2.0
    } else {
        points[m].0
    };

    // recursively divide left and right
    let (p1l, p2l, distl) = divide_and_conquer(&mut points[0..m]);
    let (p1r, p2r, distr) = divide_and_conquer(&mut points[m..n]);

    // take smaller one as reference
    let mut p1: Rc<Point>;
    let mut p2: Rc<Point>;
    let mut dist: f64;

    if distl <= distr {
        p1 = p1l;
        p2 = p2l;
        dist = distl;
    } else {
        p1 = p1r;
        p2 = p2r;
        dist = distr;
    }

    // see if points have smaller distance between partitions

    let mut j = m; // right lowest index

    for i in 0..m {
        // left index
        let pi = &points[i]; // left point

        if (pi.0 - median).abs() >= dist {
            // skip if not in x bounds
            continue;
        }

        // y bounds for right points
        let lower_y = pi.1 - dist; // lower y bounds
        let upper_y = pi.1 + dist; // upper y bounds

        // find first right point that is in bounds

        let mut pj: &Rc<Point>;

        while j < n {
            pj = &points[j];

            if (pj.0 - median).abs() >= dist {
                // skip if not in x bounds
                j += 1;
                continue;
            }

            if pj.1 > lower_y {
                // stop search if in bounds
                break;
            }
            j += 1;
        }

        // go through all possible right points in bounds

        let mut r = j;
        let mut pr: &Rc<Point>;

        while r < n {
            pr = &points[r];

            if pr.1 >= upper_y {
                // stop if out of y bounds
                break;
            }

            if (pr.0 - median).abs() >= dist {
                // skip if not in x bounds
                r += 1;
                continue;
            }

            // check dist between left and right

            let d = pi.euclidean_dist(&pr);

            if d < dist {
                // update if lower
                p1 = Rc::clone(&pi);
                p2 = Rc::clone(&pr);
                dist = d;
            }

            r += 1;
        }
    }

    // merge both partitions according to the y-position

    let mut copy: Vec<Rc<Point>> = Vec::with_capacity(n); // additional space (linear)
    copy.extend_from_slice(points);

    let mut l = 0;
    let mut r = m;

    while l < m || r < n {
        if r == n || (l < m && copy[l].1 <= copy[r].1) {
            points[l + r - m] = Rc::clone(&copy[l]);
            l += 1;
        } else {
            points[l + r - m] = Rc::clone(&copy[r]);
            r += 1;
        }
    }

    (p1, p2, dist)
}
