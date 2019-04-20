//! Crate of algorithms for solving the traveling salesman problem.
//!
//! # Example
//! ```
//! use std::time;
//!
//! use tsp_rs::Tour;
//! use tsp_rs::point::Point;
//!
//! let tour: Vec<Point> = vec![
//!     Point::new(0., 0.),
//!     Point::new(0., 1.),
//!     Point::new(1., 0.),
//!     Point::new(1., 1.),
//! ];
//!
//! let mut tour = Tour::from(&tour);
//!
//! tour.optimize_kopt(std::time::Duration::from_secs(1));
//! ```
//!
//! _Disclaimer:_
//!
//! This is not a work of art, nor is it perfect (or even good?) Rust.
//! This was written alongside my first reading of the Rust book (https://doc.rust-lang.org/book/)
//! while trying to learn the language.
mod kopt;
mod nn;
pub mod point;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::time;

use rand::Rng;

/// Trait used by all algorithms to calculate the cost of moving along an edge
///
/// # Examples
/// An example implementation is found on `tsp::point::Point`, that implements
/// standard euclidean distance as its metric.
pub trait Metrizable {
    fn cost(&self, other: &Self) -> f64;
}

/// Represents a solution to the tsp for the items T
#[derive(Debug, Clone, PartialEq)]
pub struct Tour<T: Metrizable> {
    pub path: Vec<T>,
}

impl<T: Metrizable + Clone + Borrow<T>> Tour<T> {
    /// Returns a new, empty Tour<T>
    ///
    /// # Example
    /// ```
    /// use tsp_rs::Tour;
    /// use tsp_rs::point::Point;
    ///
    /// let tour: Tour<Point> = Tour::new();
    /// ```
    pub fn new() -> Tour<T> {
        Tour {
            path: Vec::new() as Vec<T>,
        }
    }

    /// Returns a tour from `nodes: Vec<T>` passed in where the tour
    /// is nodes[0] -> nodes[1] -> ... nodes[nodes.len() - 1] -> nodes[0]
    ///
    /// # Example
    /// ```
    /// use tsp_rs::Tour;
    /// use tsp_rs::point::Point;
    ///
    /// let nodes = vec![
    ///     Point::new(0., 0.),
    ///     Point::new(1., 0.),
    ///     Point::new(1., 1.),
    ///     Point::new(0., 1.),
    /// ];
    ///
    /// let tour = Tour::from(&nodes);
    /// ```
    pub fn from(nodes: &Vec<T>) -> Tour<T>
    where
        T: Clone,
    {
        Tour {
            path: (*nodes).clone(),
        }
    }

    /// Returns the length of a tour.
    ///
    /// # Example
    /// let tour = Tour::from(&some_points);
    /// let total_cost = tour.tour_len();
    pub fn tour_len(&self) -> f64 {
        if self.path.len() <= 0 {
            return 0.;
        }

        let mut sum = 0.;
        let mut prev = self.path.last().unwrap();
        for curr in &self.path {
            sum += prev.cost(&curr);
            prev = &curr;
        }
        sum
    }

    /// Improves the tour in place using the 2opt heuristic (with 3opt kicks if it gets stuck)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time;
    ///
    /// use tsp_rs::Tour;
    /// use tsp_rs::point::Point;
    ///
    /// let nodes = vec![
    ///     Point::new(0., 0.),
    ///     Point::new(1., 0.),
    ///     Point::new(1., 1.),
    ///     Point::new(0., 1.),
    /// ];
    ///
    /// let mut tour = Tour::from(&nodes);
    ///
    /// tour.optimize_kopt(time::Duration::from_secs(1));
    /// ```
    pub fn optimize_kopt(&mut self, timeout: time::Duration) {
        self.optimize_nn();
        let start_time = time::Instant::now();
        let max_iter_withouth_impr = self.path.len() ^ 2;
        let mut iter_without_impr = 0;
        let mut best_tour_length = std::f64::MAX;
        let mut best_tour: Vec<T> = Vec::new();
        loop {
            match kopt::k_opt(2, self) {
                Some(_) => {
                    iter_without_impr = 0;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr > max_iter_withouth_impr {
                        let current_tour_length = self.tour_len();
                        if current_tour_length < best_tour_length {
                            best_tour = self.path.clone();
                            best_tour_length = current_tour_length;
                        }
                        kopt::k_opt(4, self); // kick
                        iter_without_impr = 0;
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
        let current_tour_length = self.tour_len();
        if current_tour_length < best_tour_length {
            best_tour = self.path.clone();
        }
        self.path = best_tour;
    }

    /// Constructs a tour inplace using the nearest neighbor heuristic
    ///
    /// # Examples
    ///
    /// ```
    /// use tsp_rs::Tour;
    /// use tsp_rs::point::Point;
    ///
    /// let nodes = vec![
    ///     Point::new(0., 0.),
    ///     Point::new(1., 0.),
    ///     Point::new(1., 1.),
    ///     Point::new(0., 1.),
    /// ];
    ///
    /// let mut tour = Tour::from(&nodes);
    ///
    /// tour.optimize_nn();
    /// ```
    pub fn optimize_nn(&mut self)
    where
        T: Metrizable + Clone,
    {
        let mut path = Vec::new();
        let nodes = index_path(&self.path);
        let mut visited = HashSet::new();

        let start_index: usize = rand::thread_rng().gen_range(0, nodes.len());
        let mut curr = &nodes[start_index].value.clone();
        path.push(curr.clone());
        visited.insert(nodes[start_index].index);

        loop {
            match nn::nearest_neighbor(curr, &nodes, &mut visited) {
                Some(next) => {
                    path.push(next.clone());
                    curr = &next;
                }
                None => {
                    if visited.len() == nodes.len() {
                        break;
                    }
                }
            };
        }

        self.path = path;
    }
}

#[derive(Clone)]
pub(crate) struct IndexedT<T> {
    pub index: usize,
    pub value: T,
}

#[inline]
pub(crate) fn index_path<T>(path: &Vec<T>) -> Vec<IndexedT<&T>> {
    path.iter()
        .enumerate()
        .map(|(index, value)| IndexedT { index, value })
        .collect()
}
