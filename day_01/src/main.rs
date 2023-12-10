#![allow(unused)]

use std::{
    alloc::System,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    thread::current,
    time::SystemTime,
};

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn split_by_words(line: String) -> Vec<String> {
    let mut items = Vec::<String>::new();
    let mut current_index = 0;

    while current_index < line.len() {
        let mut mem = String::new();

        for c in line.chars().skip(current_index) {
            mem.push(c);

            if WORDS.contains(&mem.as_str()) {
                break;
            }
        }

        if WORDS.contains(&mem.as_str()) {
            let numeric = WORDS.iter().position(|s| s == &mem.as_str()).unwrap() + 1;
            items.push(numeric.to_string());
            current_index += mem.len() - 1;
        } else {
            items.push(mem.chars().next().unwrap().to_string());
            current_index += 1;
        }
    }

    log::debug!("{:?}", items);

    items
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(arg) = env::args().nth(1) {
        if arg == "d" {
            env::set_var("RUST_LOG", "DEBUG");
            env_logger::init();
        }
    }

    let time_start = SystemTime::now();

    let file = File::open("data")?;
    let rdr = BufReader::new(file);

    let mut value_sum = 0;

    for l in rdr.lines() {
        let line = l.unwrap();
        let split = split_by_words(line);

        // process string
        let c = split
            .iter()
            .filter(|x| str::parse::<i32>(x).is_ok())
            .collect::<Vec<&String>>();

        let calibration_value = match (c.first(), c.last()) {
            (Some(f), Some(l)) => format!("{}{}", f, l),
            _ => panic!("Failed to parse first and last number!"),
        };

        value_sum += str::parse::<usize>(&calibration_value)?;
    }

    println!("{}", value_sum);
    println!("Time: {:?}", time_start.elapsed());

    Ok(())
}
