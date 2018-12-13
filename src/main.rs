#![feature(uniform_paths)]

mod year2018;

use std::env;
use year2018::*;

fn main() {
    let args = env::args_os();
    if args.len() == 1 {
       println!("Usage: adventofcode day01a");
    } else {
        let problem = args.into_iter().nth(1).unwrap();
        run(problem.to_str().unwrap())
    }
}

fn run(problem: &str) {
    match problem {
        "day01a" => day01a(),
        p => println!("Problem {} not found", p),
    }
}
