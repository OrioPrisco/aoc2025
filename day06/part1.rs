use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::error::Error;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn run(input : File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let input : Vec<_> = input.lines().collect::<Result<_,_>>()?;
    let (ops, input) = input.split_last().unwrap();
    let ops = ops.split_whitespace();
    let input = input.iter().map(|s| s.split_whitespace()).flatten().map(|s| s.parse::<u64>().unwrap());
    let columns = ops.clone().count();
    let results : Vec<_> = (0..columns).map(|i| input.clone().skip(i).step_by(columns)).zip(ops).map(|(iter, op)| 
        match op {
            "+" => iter.sum::<u64>(),
            "*" => iter.product(),
            _ => panic!("Unknown op {}", op)
        }
    ).collect();
    println!("{results:?}");
    println!("{}", results.iter().sum::<u64>());
    Ok(())
}
