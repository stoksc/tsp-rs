use std::borrow::Borrow;
use std::collections::HashSet;
use std::time;

use itertools::Itertools;
use rand::Rng;

use crate::common::Path;
use crate::metrizable::Metrizable;

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn solve_kopt(&mut self, timeout: time::Duration) {
        let max_iter = self.path.len() / 2;
        let mut iter_without_impr = 0;
        let mut previous_length: f64 = std::f64::MAX;
        let start_time = time::Instant::now();
        loop {
            match two_opt(self) {
                Some(x) => {
                    iter_without_impr = 0;
                    previous_length = x;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr < max_iter {
                        iter_without_impr = 0;
                        n_opt(4, self, previous_length);
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
    }
}

#[inline]
pub fn two_opt<T>(path: &mut Path<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    let p1: usize = rand::thread_rng().gen_range(0, path.path.len());
    let p2: usize = rand::thread_rng().gen_range(0, path.path.len());

    if p1 == p2 {
        return None;
    }

    let prev_len = path.path_len();
    path.path.swap(p1, p2);
    let post_len = path.path_len();

    if post_len < prev_len {
        Some(prev_len - post_len)
    } else {
        path.path.swap(p1, p2);
        None
    }
}

#[inline]
pub fn n_opt<T>(n: usize, path: &mut Path<T>, l: f64) -> Option<f64>
where
    T: Metrizable + Clone,
{
    let s = path.path.len();
    let mut unq_points: HashSet<usize> = HashSet::new();
    while unq_points.len() < n {
        let p: usize = rand::thread_rng().gen_range(0, s);
        unq_points.insert(p);
    }
    let mut swapped = Vec::new();
    for (p1, p2) in unq_points.iter().tuples() {
        swapped.push((p1.clone(), p2.clone()));
        path.path.swap(*p1, *p2);
    }
    let nl: f64 = path.path_len();
    if nl > l {
        for (p1, p2) in swapped.iter().rev() {
            path.path.swap(*p2, *p1);
        }
        return None;
    }
    return Some(nl);
}
