use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn run(input: File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let lines: Vec<_> = input.lines().collect::<Result<_, _>>()?;

    let pushes = lines.iter().map(|line| {
        let mut items = line.split_whitespace();
        let lights = items.next().ok_or("No light value")?;
        let lights = lights
            .strip_prefix("[")
            .map(|s| s.strip_suffix("]"))
            .flatten()
            .ok_or("invalid light format")?;
        let lights = lights.chars().rev().fold(0, |acc, input| {
            (acc << 1) | if input == '#' { 1 } else { 0 }
        });
        let mut buttons = Vec::new();
        while let Some(button) = items
            .by_ref()
            .map(|s| s.strip_prefix("(").map(|s| s.strip_suffix(")")))
            .flatten()
            .peekable()
            .next_if(|s| s.is_some())
        {
            buttons.push(
                button
                    .unwrap()
                    .split(",")
                    .map(str::parse::<i64>)
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        let buttons = buttons
            .iter()
            .map(|v| v.iter().fold(0, |acc, input| acc | 1 << input))
            .collect::<Vec<_>>();
        println!("{}, {:?}", lights, buttons);
        let res = CombIterator::new(buttons.len().try_into()?).map(|comb| {
            (BitIterator::new(comb).fold(0, |acc, idx| acc ^ buttons[idx]), comb.count_ones())
        })
        .filter(|&(res, _)| res == lights)
        .min_by_key(|&(_, bits)| bits);
        println!("{res:?}");
        res.map(|(_,bits)| bits).ok_or("No Combination !".into())
    }).collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    println!("{}", pushes.iter().sum::<u32>());
    Ok(())
}

type BitFieldType = u128;

struct CombIterator {
    state: BitFieldType,
    size: u8,
}

impl Iterator for CombIterator {
    type Item = BitFieldType;
    fn next(&mut self) -> Option<Self::Item> {
        //self.state = dbg!(next_bit(self.state, 0, self.size))?;
        if self.state == (1 << self.size) - 1 {
            return None
        }
        self.state += 1;
        Some(self.state)
    }
}

impl CombIterator {
    pub fn new(size : u8) -> Self {
        assert!(size <= 128);
        CombIterator{state: 0, size}
    }
}

struct BitIterator {
    state: BitFieldType,
}

impl Iterator for BitIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            None
        } else {
            let res = self.state.trailing_zeros();
            self.state ^= 1 << res;
            //println!("{res}");
            Some(res as usize)
        }
    }
}

impl BitIterator {
    pub fn new(state: BitFieldType) -> Self {
        Self{state}
    }
}
