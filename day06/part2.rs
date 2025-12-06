use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::error::Error;
use std::iter;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn run(input : File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let input : Vec<_> = input.lines().collect::<Result<_,_>>()?;
    let (ops, input) = input.split_last().unwrap();
    let mut ops = ops.split_whitespace();
    let columns = input[0].len();
    let input = input.iter().map(|s| s.chars()).flatten();
    let input : Vec<_> = (0..columns).map(|i| input.clone().skip(i).step_by(columns).collect::<String>()).collect();
    println!("{input:?}");
    let mut numbers : Vec<u64> = Vec::new();
    let mut results : Vec<u64> = Vec::new();
    for string in input.iter().chain(iter::once(&String::from(" "))) {
        if string.chars().all(char::is_whitespace) {
            let op = ops.next().unwrap();
            results.push(match op {
                "+" => numbers.iter().sum(),
                "*" => numbers.iter().product(),
                _ => panic!("Unknown op {}", op),
            });
            numbers.clear();
        } else {
            numbers.push(string.trim().parse()?);
        }
    }
    println!("{results:?}");
    println!("{}", results.iter().sum::<u64>());
    Ok(())
}
