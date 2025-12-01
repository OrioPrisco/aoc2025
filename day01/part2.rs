use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    let input : Vec<_> = BufReader::new(input).lines().collect::<Result<_,_>>().unwrap();

    let codes : Vec<_> = input.iter().map(|s| s.split_at(1)).collect();
    let positions : Vec<i64> = codes.iter().scan(50, |dial : &mut i64, code| { 
        let magnitude = code.1.parse::<i64>().unwrap();
        let direction = if code.0 == "R" {1} else {-1};
        let mut res = 0;
        for _ in 0..magnitude {
            *dial += direction;
            *dial = dial.rem_euclid(100);
            if *dial == 0 {
                res += 1;
            }
        }
        Some(res)
    }
    ).collect();
    println!("{positions:?}");
    println!("{}", positions.iter().sum::<i64>());
}
