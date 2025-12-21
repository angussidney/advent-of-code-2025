use phf::phf_map;
use std::fs;
use std::{env, process::exit, str::Lines};

mod day1a;

static DAYS: phf::Map<&str, fn(Lines) -> usize> = phf_map! {
    "day1a" => day1a::run,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: advent-of-code <day> <testcase_type>");
        exit(1);
    }
    let day = &args[1];
    let testcase_type = &args[2];

    if !DAYS.contains_key(&day) {
        println!("{day} has no implementation currently.");
        exit(1);
    }

    if !(testcase_type == "sample" || testcase_type == "challenge") {
        println!("{testcase_type} is not a valid testcase type");
        exit(1);
    }

    let file_contents = fs::read_to_string(format!("tests/{day}/{testcase_type}.txt"))
        .expect("Input files not present for chosen day!");
    let test_data = file_contents.lines();

    if let Some(func) = DAYS.get(&day) {
        println!("Running {day} with input file {testcase_type}.txt\n");

        let result = func(test_data);
        println!("{result}");
    } else {
        println!("{day} has no implementation currently.");
        exit(1);
    }
}
