use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn run(input: File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let lines: Vec<_> = input.lines().collect::<Result<_, _>>()?;
    let coords : Vec<_> = lines
        .iter()
        .map(|s| -> Result<(i64, i64), Box<dyn Error>> {
            let (x, y) = s.split_once(',').ok_or("no `,` found in line")?;
            Ok((x.parse::<i64>()?, y.parse::<i64>()?))
        })
        .collect::<Result<_,_>>()?;
    println!("{:?}", coords);

    let mut pairs = coords
        .iter()
        .enumerate()
        .map(|(i, &c1)| {
            coords
                .iter()
                .skip(i + 1)
                .map(move |&c2| (c1, c2, ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs()+1) ))
        })
        .flatten()
        .collect::<Vec<_>>();
    pairs.sort_by_key(|(_,_, area)| *area);
    println!("{:?}", pairs.last().ok_or("empty input")?);

    Ok(())
}
