use std::borrow::Borrow;
use std::collections::HashSet;
use std::time;

use itertools::Itertools;
use rand::Rng;

use crate::common::{self, Path};
use crate::metrizable::Metrizable;

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn solve_kopt(&mut self, timeout: time::Duration) {
        let max_iter = self.path.len() / 2;
        let mut iter_without_impr = 0;
        let mut previous_length: f64 = std::f64::MAX;
        let start_time = time::Instant::now();
        loop {
            match crate::kopt::two_opt(&mut self.path) {
                Some(x) => {
                    iter_without_impr = 0;
                    previous_length = x;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr < max_iter {
                        iter_without_impr = 0;
                        crate::kopt::n_opt(4, &mut self.path, previous_length);
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
        self.len = common::length(&self.path);
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

    let prev_len = common::length(&v);
    v.swap(p1, p2);

    if common::length(&v) < prev_len {
        Some(prev_len - common::length(&v))
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
    let nl: f64 = common::length(v);
    if nl > l {
        for (p1, p2) in swapped.iter().rev() {
            v.swap(*p2, *p1);
        }
        return None;
    }
    return Some(nl);
}
