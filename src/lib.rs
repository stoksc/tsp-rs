use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;
use std::time;

const MAX_ITER_WITHOUT_IMPR: u32 = 100;

pub trait Metrizable {
    fn distance(&self, y: &Self) -> f64;
}

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

impl Metrizable for Point {
    fn distance(&self, b: &Point) -> f64 {
        return ((self.x - b.x).powf(2.) + (self.y - b.y).powf(2.)).sqrt();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path<T: Metrizable> {
    pub len: f64,
    pub path: Vec<T>,
}

pub fn solve<T>(v: &Vec<T>, timeout: time::Duration) -> Path<T>
where
    T: Metrizable + Clone,
{
    let mut result = v.clone();
    let mut iter_without_impr = 0;
    let mut past: f64 = std::f64::MAX;
    let start_time = time::Instant::now();
    loop {
        match two_opt(&mut result) {
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
        path: result.to_vec(),
    }
}

#[inline]
pub fn two_opt<T>(v: &mut Vec<T>) -> Option<f64>
where
    T: Metrizable,
{
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
pub fn n_opt<T>(n: usize, v: &mut Vec<T>, l: f64) -> Option<f64>
where
    T: Metrizable,
{
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
pub fn length<T>(v: &Vec<T>) -> f64
where
    T: Metrizable,
{
    if v.len() <= 0 {
        return 0.;
    }

    let mut sum = 0.;
    let mut prev = v.last().unwrap();
    for curr in v {
        sum += prev.distance(curr);
        prev = curr;
    }
    sum
}
