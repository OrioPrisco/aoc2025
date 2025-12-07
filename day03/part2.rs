use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::error::Error;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn best_joltage(mut line: impl Iterator<Item=char> + Clone, progress: String, mut current_best: String, size_left: usize) -> String {
    if size_left == 0 {
        return progress;
    }
    let mut max_seen = '0';
    while let Some(c) = line.next() {
        if c < max_seen {
            continue;
        }
        if line.clone().count() < size_left - 1 {
            return current_best;
        }
        max_seen = c;
        let mut progress = progress.clone();
        progress.push(c);
        if &current_best[0..progress.len()] > progress.as_str() {
            continue;
        }
        current_best = best_joltage(line.clone(), progress, current_best, size_left - 1);
    }
    current_best
}

fn run(input : File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let input : Vec<_> = input.lines().collect::<Result<_,_>>()?;
    println!("{input:?}");
    let joltages = input.iter().map(|s| best_joltage(s.chars(), String::new(), String::from("000000000000"), 12)).collect::<Vec<_>>();
    println!("{joltages:?}");
    println!("{}", joltages.iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>());
    Ok(())
}
