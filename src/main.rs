#![feature(uniform_paths)]

mod utils;
mod year2018;

use std::env;
use std::io;
use utils::read_input;
use year2018::*;

fn main() -> io::Result<()> {
    let args = env::args_os();
    if args.len() == 1 {
       println!("Usage: adventofcode day01a");
       Ok(())
    } else {
        let problem = args.into_iter().nth(1).unwrap();
        run(problem.to_str().unwrap())
    }
}

fn run(problem: &str) -> io::Result<()> {
    match problem {
        "day01a" => {
          println!("{}", day01a(read_input("input/2018/day01")?));
          Ok(())
        },
        "day01b" => {
          println!("{:?}", day01b(read_input("input/2018/day01")?));
          Ok(())
        },
        p => {
            println!("Problem {} not found", p);
            Ok(())
        },
    }
}
