extern crate getopts;
extern crate rustc_serialize;
extern crate csv;

use getopts::Options;
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Debug, RustcDecodable)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

struct PopulationCount {
    city: String,
    country: String,
    count: u64,
}

impl PopulationCount {
    fn new(city: String, country: String, count: u64) -> PopulationCount {
        PopulationCount {
            city: city,
            country: country,
            count: count,
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options] <data-path> <city>", program)));
}

fn search<P: AsRef<Path>>(file_path: P, city: &str) -> Vec<PopulationCount> {
    let mut found = vec![];
    let file = File::open(file_path).unwrap();
    let mut reader = csv::Reader::from_reader(file);

    for row in reader.decode::<Row>() {
        let row = row.unwrap();

        match row.population {
            None => {},
            Some(count) => if row.city == city {
                found.push(
                    PopulationCount::new(row.city, row.country, count));
            },
        }
    }

    found
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string())},
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let data_path = &matches.free[0];
    let city: &str = &matches.free[1];

    for pop in search(data_path, city) {
        println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    }
}
