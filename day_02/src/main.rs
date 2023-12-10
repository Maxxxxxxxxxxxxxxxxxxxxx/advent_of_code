#![allow(unused)]

use core::num;
use std::{env, error::Error, fs::File, io::BufRead, io::BufReader, time::SystemTime};

use regex::Regex;

const MAX_VALUES: [u64; 3] = [12, 13, 14];
const REGEX_PATTERNS: [&str; 3] = [
    "([1-9][0-9]* red)",
    "([1-9][0-9]* green)",
    "([1-9][0-9]* blue)",
];

// Part one function
fn check_game(line: &str) -> Result<bool, Box<dyn Error>> {
    for (i, pattern) in REGEX_PATTERNS.iter().enumerate() {
        let re = Regex::new(pattern)?;

        for mtch in re.find_iter(line) {
            // log::debug!("Found match: {}", mtch.as_str());

            let number = mtch.as_str().split(' ').next().unwrap_or("0");
            let parsed = str::parse::<u64>(number)?;

            if parsed > MAX_VALUES[i] {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

// Part two function
fn find_minimum(line: &str) -> Result<[u64; 3], Box<dyn Error>> {
    let mut values: [u64; 3] = Default::default();

    for (i, pattern) in REGEX_PATTERNS.iter().enumerate() {
        let re = Regex::new(pattern)?;

        for mtch in re.find_iter(line) {
            let number = mtch.as_str().split(' ').next().unwrap_or("0");
            let parsed = str::parse::<u64>(number)?;

            if parsed > values[i] {
                values[i] = parsed;
            }
        }
    }

    Ok(values)
}

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1) {
        Some(a) if a == "d" => {
            env::set_var("RUST_LOG", "DEBUG");
            env_logger::init();
        }
        _ => (),
    }

    let timestamp = SystemTime::now();
    let f = File::open("data")?;
    let rdr = BufReader::new(f);

    let mut gameid_sum = 0;
    let mut power_sum = 0;

    for (index, l) in rdr.lines().enumerate() {
        let line = l.unwrap();

        // Part one task
        if check_game(&line)? {
            log::debug!("Game {} is possible", index + 1);
            gameid_sum += index + 1;
        }

        // Part two task
        let m = find_minimum(&line)?;
        log::debug!("Game {} m: {:?}", index + 1, &m);
        power_sum += m[0] * m[1] * m[2]; // Summing the power of minimum values
    }

    println!("Part 1 output: {}", gameid_sum);
    println!("Part 2 output: {}", power_sum);
    println!("Elapsed: {:?}", timestamp.elapsed());

    Ok(())
}
