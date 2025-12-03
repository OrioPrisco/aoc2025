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
    println!("{input:?}");
    let joltage : Vec<_> = input.iter().map(|s| {
        (1..s.len()).map(|i| {
            let (head,tail) = s.split_at(i);
            tail.chars().map(|c| 
            head.chars().last().unwrap().to_digit(10).unwrap() * 10 + 
            c.to_digit(10).unwrap()).max().unwrap()
        }).max().unwrap()
    }).collect();
    println!("{joltage:?}");
    println!("{:?}", joltage.iter().clone().sum::<u32>());
    Ok(())
}
