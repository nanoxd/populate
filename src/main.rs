extern crate getopts;
extern crate rustc_serialize;
extern crate csv;

use getopts::Options;
use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io;

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
    println!("{}", opts.usage(&format!("Usage: {} [options] <city>", program)));
}

fn search<P: AsRef<Path>>(file_path: &Option<P>, city: &str) -> Result<Vec<PopulationCount>, Box<Error>> {
    let mut found = vec![];

    let input: Box<io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(try!(File::open(file_path)))
    };

    let mut reader = csv::Reader::from_reader(input);

    for row in reader.decode::<Row>() {
        let row = try!(row);

        match row.population {
            None => {},
            Some(count) => if row.city == city {
                found.push(
                    PopulationCount::new(row.city, row.country, count));
            },
        }
    }

    if found.is_empty() {
        Err(From::from("No matching criteria with a population were found."))
    } else {
        Ok(found)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("f", "file", "Choose an input file, instead of using STDIN", "NAME");
    opts.optflag("h", "help", "Show this usage message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string())},
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let data_path = &matches.opt_str("f");

    let city: &str = if !matches.free.is_empty() {
        &matches.free[0]
    } else {
        print_usage(&program, opts);
        return;
    };

    match search(&data_path, city) {
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        },
        Err(err) => println!("{}", err)
    }
}
