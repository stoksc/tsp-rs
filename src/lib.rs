use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;
use std::time;

const MAX_ITER_WITHOUT_IMPR: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub len: f64,
    pub path: Vec<Point>,
}

pub fn solve(v: &Vec<Point>, timeout: time::Duration) -> Path {
    let mut result = v.clone();
    let mut iter_without_impr = 0;
    let mut past: f64 = std::f64::MAX;
    let start_time = time::Instant::now();
    loop {
        let impr: Option<f64> = two_opt(&mut result);
        match impr {
            Some(x) => {
                iter_without_impr = 0;
                past = x;
            }
            None => {
                iter_without_impr += 1;
                if iter_without_impr < MAX_ITER_WITHOUT_IMPR {
                    iter_without_impr = 0;
                    n_opt(4, &mut result, past);
                }
            }
        }
        if start_time.elapsed() > timeout {
            break;
        }
    }
    Path {
        len: length(&result),
        path: result,
    }
}

#[inline]
pub fn two_opt(v: &mut Vec<Point>) -> Option<f64> {
    let p1: usize = rand::thread_rng().gen_range(0, v.len());
    let p2: usize = rand::thread_rng().gen_range(0, v.len());

    if p1 == p2 {
        return None;
    }

    let prev_len = length(&v);
    v.swap(p1, p2);
    if length(&v) < prev_len {
        Some(prev_len - length(&v))
    } else {
        v.swap(p1, p2);
        None
    }
}

#[inline]
pub fn n_opt(n: usize, v: &mut Vec<Point>, l: f64) -> Option<f64> {
    let s = v.len();
    let mut unq_points: HashSet<usize> = HashSet::new();
    while unq_points.len() < n {
        let p: usize = rand::thread_rng().gen_range(0, s);
        unq_points.insert(p);
    }
    let mut swapped = Vec::new();
    for (p1, p2) in unq_points.iter().tuples() {
        swapped.push((p1.clone(), p2.clone()));
        v.swap(*p1, *p2);
    }
    let nl: f64 = length(v);
    if nl > l {
        for (p1, p2) in swapped.iter().rev() {
            v.swap(*p2, *p1);
        }
        return None;
    }
    return Some(nl);
}

#[inline]
pub fn length(v: &Vec<Point>) -> f64 {
    if v.len() <= 0 {
        return 0.;
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
    return ((a.x - b.x).powf(2.) + (a.y - b.y).powf(2.)).sqrt();
}
