use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let input : Vec<_> = contents.trim().split(",").map(|s| s.split_once("-").unwrap()).collect();
    println!("{:?}", input);
    let duplicates : Vec<_> = input.iter().map(|(begin, end)| {
        let begin = begin.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();
        let mut res = 0;
        for i in begin..=end {
            let digits = i.ilog10() + 1;
            for j in 2..=digits {
                if digits % j != 0 { continue; }
                let digits = digits / j;
                let power = 10i64.pow(digits);
                let mut numbers = (0..j).map(|j| i / power.pow(j) % power);
                let first = numbers.next().unwrap();
                if numbers.all(|n| n == first) {
                    res += i;
                    break;
                }
            }
        }
        res
    }).collect();
    println!("{:?}", duplicates);
    println!("{:?}", duplicates.iter().sum::<i64>());
}
