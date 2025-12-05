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
    let mut ranges : Vec<_> = ranges.iter().map(|s| {
        let (begin,end) = s.split_once("-").unwrap();
        begin.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    }).collect();
    ranges.sort_by_key(|r| *r.start());
    println!("{ranges:?}");
    let mut all_fresh = Vec::new();
    all_fresh.push(ranges[0].clone());
    for range in ranges {
        let acc = all_fresh.last_mut().unwrap();
        if acc.end() < range.start() {
            all_fresh.push(range.clone());
            continue;
        }
        *acc = *acc.start()..=*range.end().max(acc.end());
    }
    println!("{all_fresh:?}");
    let all_ids : u64 = all_fresh.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("{all_ids}");
    Ok(())
}
