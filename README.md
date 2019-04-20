# tsp-rs [![CircleCI](https://circleci.com/gh/stoksc/tsp-rs.svg?style=svg)](https://circleci.com/gh/stoksc/tsp-rs)

Library for traveling salesman problem algorithms. 

## Example

### Basic

For 2d point datasets:

```rust
use std::time;

use tsp_rs::Tour;
use tsp_rs::point::Point;

let tour: Vec<Point> = vec![
    Point::new(0., 0.),
    Point::new(0., 1.),
    Point::new(1., 0.),
    Point::new(1., 1.),
];

let mut tour = Tour::from(&tour);

tour.solve_kopt(std::time::Duration::from_secs(1));
```

### Using traits

Same as above, but instead of using `tsp::point::Point`, just implement the trait `tsp::metrizable::Metrizable`
for your type `T` by defining a distance function between two `T`. Your type will also need `Clone`, `Borrow`, maybe another.. the compiler will remember.

## Performance

`Path::solve_kopt` uses a 2-opt heuristic with 3-opt thrown in if it hits a wall for too long. Gets to within ~8% of the optimal solution for the b52 and ~10% of qa194 on average in a run of solve_nn + 1 second of optimization. The larger the problem, the longer you should allow for optimization.

For the constructive solution, `Path::solve_nn`, gets to within ~15% of the optimal solution on average.

## Comments

Just for my own entertainment while learning rust, don't trust this but the implementation should be correct.