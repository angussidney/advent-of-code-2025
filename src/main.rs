use phf::phf_map;
use std::fs;
use std::{env, process::exit, str::Lines};

mod common;
mod day1a;
mod day1b;
mod day2a;
mod day2b;
mod day3a;
mod day3b;
mod day4a;
mod day4b;
mod day5a;
mod day5b;
mod day6a;
mod day6b;

static DAYS: phf::Map<&str, fn(Lines) -> usize> = phf_map! {
    "day1a" => day1a::run,
    "day1b" => day1b::run,
    "day2a" => day2a::run,
    "day2b" => day2b::run,
    "day3a" => day3a::run,
    "day3b" => day3b::run,
    "day4a" => day4a::run,
    "day4b" => day4b::run,
    "day5a" => day5a::run,
    "day5b" => day5b::run,
    "day6a" => day6a::run,
    "day6b" => day6b::run,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: advent-of-code <day> <testcase_type>");
        exit(1);
    }
    let day_variant = &args[1];
    let day = day_variant.get(..day_variant.len() - 1).unwrap();
    let testcase_type = &args[2];

    if !DAYS.contains_key(&day_variant) {
        println!("{day_variant} has no implementation currently.");
        exit(1);
    }

    if !(testcase_type == "sample" || testcase_type == "challenge") {
        println!("{testcase_type} is not a valid testcase type");
        exit(1);
    }

    let file_contents = fs::read_to_string(format!("tests/{day}/{testcase_type}.txt"))
        .expect("Input files not present for chosen day!");
    let test_data = file_contents.lines();

    if let Some(func) = DAYS.get(&day_variant) {
        println!("Running {day_variant} with input file {testcase_type}.txt\n");

        let result = func(test_data);
        println!("{result}");
    } else {
        println!("{day_variant} has no implementation currently.");
        exit(1);
    }
}
