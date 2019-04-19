# tsp

Library for traveling salesman problem algorithms.

## Example

### Basic

For 2d point datasets:

```rust
let my_data: Vec<(f64, f64)> = ... ;

// convert your data to Point structs
let path: Vec<tsp::point::Point> = my_data.iter().map(|(x, y)| {
    tsp::point::Point{ x, y }
}).collect();

// construct a Path
let mut path = tsp::common::Path::from(&path);

// call solve with a timeout specified
path.solve_kopt(std::time::Duration::from_secs(1));

// or use a constructive solution (faster but worse)
path.solve_nn();
```

### Using traits

Same as above, but instead of using `tsp::point::Point`, just implement the trait `tsp::metrizable::Metrizable`
for your type `T` by defining a distance function between two `T`. Your type will also need `Clone`, `Borrow`, maybe another.. the compiler will remember.

## Performance

`Path::solve_kopt` uses a 2-opt heuristic with 3-opt thrown in if it hits a wall for too long. Gets to within ~8% of the optimal solution for the berlin52 on average in 1 second of execution.

For the constructive solution, `Path::solve_nn`, gets to within ~22% of the optimal solution on average.

## Comments

Just for my own entertainment while learning rust, don't trust this but the implementation should be correct.