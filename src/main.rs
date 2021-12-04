use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

use clap::Parser;

extern crate clap;
mod challenges;

#[derive(Parser)]
#[clap(version = "1.0", author = "Carson Myers <carson@myers.se>")]
struct Opts {
    day: u8,
    part: u8,

    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let filename = vec![
        opts.file,
        Some(format!("input/day{}_part{}", opts.day, opts.part)),
        Some(format!("input/day{}", opts.day)),
    ].into_iter()
        .filter_map(|filename| filename)
        .find(|filename| Path::new(filename).exists())
        .expect("no input file for challenge");

    let input = File::open(filename)
        .map(|file| io::BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>())
        .expect("cannot read input file");


    let challenge = challenges::get_challenge(opts.day);
    let result = match opts.part {
        1 => challenge.part_1(input),
        2 => challenge.part_2(input),
        p @ _ => panic!("invalid part: {}", p),
    };

    println!("result: {}", result);
}
