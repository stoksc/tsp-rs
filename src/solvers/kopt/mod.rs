use std::borrow::Borrow;
use std::time;

use rand::Rng;

use crate::common::Tour;
use crate::metrizable::Metrizable;

impl<T: Metrizable + Clone + Borrow<T>> Tour<T> {
    pub fn solve_kopt(&mut self, timeout: time::Duration) {
        self.solve_nn();
        let start_time = time::Instant::now();
        let max_iter_withouth_impr = self.path.len() ^ 2;
        let mut iter_without_impr = 0;
        let mut best_tour_length = std::f64::MAX;
        let mut best_tour: Vec<T> = Vec::new();
        loop {
            match k_opt(2, self) {
                Some(_) => {
                    iter_without_impr = 0;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr > max_iter_withouth_impr {
                        let current_tour_length = self.path_len();
                        if current_tour_length < best_tour_length {
                            best_tour = self.path.clone();
                            best_tour_length = current_tour_length;
                        }
                        k_opt(4, self); // kick
                        iter_without_impr = 0;
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
        let current_tour_length = self.path_len();
        if current_tour_length < best_tour_length {
            best_tour = self.path.clone();
        }
        self.path = best_tour;
    }
}

#[inline]
pub fn k_opt<T>(k: usize, path: &mut Tour<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    match k {
        2 => {
            let mut i = rand_index(path);
            let mut j = rand_index(path);

            if i == j {
                return None;
            }

            let mut ij = vec![i, j];
            ij.sort();
            i = ij[0];
            j = ij[1];

            two_opt(i, j, path)
        }
        3 => {
            let mut i = rand_index(path);
            let mut j = rand_index(path);
            let mut k = rand_index(path);

            if i == j || j == k {
                return None;
            }

            let mut ijk = vec![i, j, k];
            ijk.sort();
            i = ijk[0];
            j = ijk[1];
            k = ijk[2];

            three_opt(i, j, k, path)
        }
        4 => {
            let mut i = rand_index(path);
            let mut j = rand_index(path);
            let mut k = rand_index(path);
            let mut l = rand_index(path);

            if i == j || j == k || k == l {
                return None;
            }

            let mut ijkl = vec![i, j, k, l];
            ijkl.sort();
            i = ijkl[0];
            j = ijkl[1];
            k = ijkl[2];
            l = ijkl[3];

            four_opt(i, j, k, l, path)
        }
        _ => panic!("Not implemented"),
    }
}

#[inline]
pub fn two_opt<T>(i: usize, j: usize, path: &mut Tour<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    let mut new_path = Vec::from(&path.path[..i]);
    let mut middle = Vec::from(&path.path[i..j]);
    middle.reverse();
    new_path.append(&mut middle);
    new_path.append(&mut Vec::from(&path.path[j..]));

    let new_path = Tour { path: new_path };
    let prev_len = path.path_len();
    let post_len = new_path.path_len();

    if post_len < prev_len {
        path.path = new_path.path;
        Some(post_len - prev_len)
    } else {
        None
    }
}

#[inline]
pub fn three_opt<T>(i: usize, j: usize, k: usize, path: &mut Tour<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    Some(two_opt(i, j, path).unwrap_or_default() + two_opt(j, k, path).unwrap_or_default())
}

#[inline]
pub fn four_opt<T>(i: usize, j: usize, k: usize, l: usize, path: &mut Tour<T>) -> Option<f64>
where
    T: Metrizable + Clone,
{
    Some(
        two_opt(i, j, path).unwrap_or_default()
            + two_opt(j, k, path).unwrap_or_default()
            + two_opt(k, l, path).unwrap_or_default(),
    )
}

pub fn rand_index<T>(path: &Tour<T>) -> usize
where
    T: Metrizable,
{
    rand::thread_rng().gen_range(0, path.path.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Tour;
    use crate::point::Point;

    #[test]
    fn test_two_opt() {
        let mut path = Tour::from(&vec![
            Point::new(0., 0.),
            Point::new(1., 1.),
            Point::new(1., 0.),
            Point::new(0., 1.),
        ]);

        let two_opt_path = Tour::from(&vec![
            Point::new(0., 0.),
            Point::new(1., 0.),
            Point::new(1., 1.),
            Point::new(0., 1.),
        ]);

        let result = two_opt(1, 3, &mut path);

        assert_ne!(None, result);
        assert_eq!(path, two_opt_path);
    }
}
