extern crate clap;
mod challenges;

use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use clap::Parser;

use crate::challenges::Challenge;

#[derive(Parser)]
#[clap(version = "1.0", author = "Carson Myers <carson@myers.se>")]
struct Opts {
    #[clap(short, long, default_value = "1")]
    part: u8,
    day: u8,

    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.day < 1 || opts.day > 25 {
        eprintln!("Invalid day: {}", opts.day);
        process::exit(1)
    }

    let filename = match opts.file {
        Some(f) => f,
        None => format!("input/day{}_part{}", opts.day, opts.part)
    };

    let challenge = challenges::get_challenge(opts.day);

    let result = match File::open(filename.clone())
        .map(|file| io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>())
    {
        Ok(input) => match opts.part {
            1 => challenge.part_1(input),
            2 => challenge.part_2(input),
            n @ _ => {
                eprintln!("Invalid challenge part: {}", n);
                process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Could not open input file {}: {}", filename, error);
            process::exit(1);
        }
    };

    println!("result: {}", result);
}
