use std::fs::File;
use std::io::Read;
use std::time;

const TEST_DATA_FILENAMES: [&str; 2] = ["tests/data/b52.tsp", "tests/data/qa194.tsp"];
const DEFAULT_TIMEOUT: u64 = 1;
const EOF: &str = "EOF";

#[test]
fn test_solve() {
    for filename in &TEST_DATA_FILENAMES {
        let filename = String::from(*filename);
        let v = parse_tsp_file(&filename);

        let timeout = time::Duration::from_secs(DEFAULT_TIMEOUT);
        let mut path = tsp::common::Path::from(&v);
        path.solve_kopt(timeout);
        println!("solve_kopt on {} had length {}", filename, path.path_len());
    }
}

#[test]
fn test_solve_nn() {
    for filename in &TEST_DATA_FILENAMES {
        let filename = String::from(*filename);
        let v = parse_tsp_file(&filename);

        let mut path = tsp::common::Path::from(&v);
        path.solve_nn();
        println!("solve_nn on {} had length {}", filename, path.path_len());
    }
}

fn parse_tsp_file(filename: &String) -> Vec<tsp::point::Point> {
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
        let point = tsp::point::Point::new(parse_next_float(), parse_next_float());
        results.push(point);
    }
    results
}
