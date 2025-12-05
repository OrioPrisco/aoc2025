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
    let separator = input.iter().position(|s| s.is_empty()).unwrap();
    let ranges = &input[0..separator];
    let ids = &input[separator+1..];
    let ranges : Vec<_> = ranges.iter().map(|s| {
        let (begin,end) = s.split_once("-").unwrap();
        begin.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    }).collect();
    println!("{ranges:?}");
    let ids : Vec<_> = ids.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    println!("{ids:?}");
    let fresh = ids.iter().map(|id| ranges.iter().any(|r| r.contains(id))).filter(|b| *b).count();
    println!("{fresh}");
    Ok(())
}
