use std::fs::File;
use std::io::Read;
use std::time;

use tsp_rs::point::Point;
use tsp_rs::Tour;

const TEST_DATA_FILENAMES: [&str; 3] = [
    "tests/data/b52.tsp",
    "tests/data/qa194.tsp",
    "tests/data/vm22775.tsp",
];
const DEFAULT_TIMEOUT: u64 = 1;
const EOF: &str = "EOF";

#[test]
fn test_solve() {
    for filename in &TEST_DATA_FILENAMES {
        let n = 1;
        let mut total = 0.;
        let mut best = std::f64::MAX;
        let mut worst = std::f64::MIN;
        for _ in 0..n {
            let filename = String::from(*filename);
            let v = parse_tsp_file(&filename);

            let timeout = time::Duration::from_secs(DEFAULT_TIMEOUT);
            let mut tour = Tour::from(&v);
            tour.optimize_kopt(timeout);

            let result = tour.tour_len();
            total += result;
            best = if result < best { result } else { best };
            worst = if result > worst { result } else { worst };
        }
        println!("solve_kopt on {} had:", filename);
        println!("\tworst case: {}", worst);
        println!("\tbest case: {}", best);
        println!("\taverage: {}", total / (n as f64));
    }
}

#[test]
fn test_solve_nn() {
    for filename in &TEST_DATA_FILENAMES {
        let filename = String::from(*filename);
        let v = parse_tsp_file(&filename);

        let mut tour = Tour::from(&v);
        tour.optimize_nn();
        println!("solve_nn on {} had length {}", filename, tour.tour_len());
    }
}

fn parse_tsp_file(filename: &String) -> Vec<Point> {
    let mut f = File::open(filename).expect("Failed to open test data file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Failed to read file into string");

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(EOF) {
            break;
        }

        let mut words = line.split_whitespace().skip(1); //
        let mut parse_next_float = || match words.next() {
            Some(x) => x.trim().parse().expect("Failed to parse point"),
            None => panic!("Not enough data to parse point"),
        };
        let point = Point::new(parse_next_float(), parse_next_float());
        results.push(point);
    }
    results
}
