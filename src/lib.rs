extern crate itertools;

#[cfg(test)]
pub mod tests {
    use crate::core::{Point};
    use std::time::SystemTime;

    #[test]
    fn it_works() {
        let filename = String::from("data/b52.tsp");
        let mut v = crate::data::parse_file(&filename);

        let t0 = SystemTime::now();
        let (l, _sol): (f64, Vec<Point>) = crate::core::solve(&mut v);
        println!("{:?}", SystemTime::now().duration_since(t0));
        assert_eq!(l, 10000.);
    }
}

pub mod core {
    use std::hash::{Hash, Hasher};
    use std::collections::HashSet;
    use rand::Rng;
    use itertools::Itertools;

    const MAX_ITER_WITHOUT_IMPR: u32 = 5;
    const TOTAL_ITER: u32 = 100_000;

    #[derive(Debug, Clone, Copy)]
    pub struct Point(pub u32, pub f64, pub f64);

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.1 == other.1 && self.2 == other.2
        }
    }
    
    impl Eq for Point {}

    impl Hash for Point {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    pub fn solve(v: &mut Vec<Point>) -> (f64, Vec<Point>) {
        let mut iter_without_impr = 0;
        let mut past: f64 = std::f64::MAX;
        for _ in 1..TOTAL_ITER {
            let impr: Option<f64> = n_opt(2, v, past);
            match impr {
                Some(x) => {
                    iter_without_impr = 0;
                    past = x;
                },
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr < MAX_ITER_WITHOUT_IMPR {
                        iter_without_impr = 0;
                        n_opt(4, v, past);
                    }
                }
            }
        }
        (length(v), v.clone())
    }

    #[inline]
    pub fn n_opt(n: usize, v: &mut Vec<Point>, l: f64) -> Option<f64> {
        let s = v.len();
        let mut unq_points: HashSet<usize> = HashSet::new();
        while unq_points.len() < n {
            let p: usize = rand::thread_rng().gen_range(0, s);
            unq_points.insert(p);
        }
        for (p1, p2) in unq_points.iter().tuples() {
            v.swap(*p1, *p2);
        }
        let nl: f64 = length(v);
        if nl > l {
            for (p1, p2) in unq_points.iter().tuples() {
                v.swap(*p2, *p1);
            }
            return None;
        }
        return Some(nl)
    }

    #[inline]
    pub fn length(v: & Vec<Point>) -> f64 {
        if v.len() <= 0 {
            return 0.
        }

        let mut sum = 0.;
        let mut prev = v.last().unwrap();
        for curr in v {
            sum += euc_dist(&prev, curr);
            prev = curr;
        }
        sum
    }

    #[inline]
    pub fn euc_dist(a: &Point, b: &Point) -> f64 {
        return ((a.1 - b.1).powf(2.) + (a.2 - b.2).powf(2.)).sqrt()
    }
}

pub mod data {
    use itertools::Itertools;
    use crate::core::Point;

    pub fn parse_file(filename: &String) -> Vec<Point> {
        let file_err = "Something went wrong reading the file";
        let int_parse_err = "Something went wrong parsing a point";
        let contents = std::fs::read_to_string(filename).expect(file_err);
        let mut d: Vec<Point> = Vec::new();
        for chunk in &contents.split_whitespace().chunks(3) {
            let mut p = Point(0, 0., 0.);
            for (i, c) in chunk.enumerate() {
                if c == "EOF" {
                    return d;
                }
                match i {
                    0 => p.0 = c.trim().parse().expect(int_parse_err),
                    1 => p.1 = c.trim().parse().expect(int_parse_err),
                    2 => p.2 = c.trim().parse().expect(int_parse_err),
                    _ => continue,
                }
            }
            d.push(p);
        }
        d
    }
}