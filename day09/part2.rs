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
    let coords: Vec<_> = lines
        .iter()
        .map(|s| -> Result<(i64, i64), Box<dyn Error>> {
            let (x, y) = s.split_once(',').ok_or("no `,` found in line")?;
            Ok((x.parse::<i64>()?, y.parse::<i64>()?))
        })
        .collect::<Result<_, _>>()?;
    println!("{:?}", coords);

    let coords_looped = coords
        .iter()
        .chain(coords.iter())
        .take(coords.len() + 1)
        .collect::<Vec<_>>();
    let avoid = coords_looped.windows(2);

    let mut pairs = coords
        .iter()
        .enumerate()
        .map(|(i, &c1)| {
            coords.iter().skip(i + 1).map(move |&c2| {
                (
                    c1,
                    c2,
                    ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1),
                )
            })
        })
        .flatten()
        .filter(|(c1, c2, _)| {
            let x1 = c1.0.min(c2.0);
            let x2 = c1.0.max(c2.0);
            let y1 = c1.1.min(c2.1);
            let y2 = c1.1.max(c2.1);
            coords_looped.windows(2).find(|win| {
                let d1 = win[0];
                let d2 = win[1];
                if d1.0 == d2.0 {
                    let start = d1.1.min(d2.1);
                    let end = d1.1.max(d2.1);
                    x1 < d1.0
                        && d1.0 < x2
                        && !((start <= y1 && end <= y1) || (start >= y2 && end >= y2))
                } else {
                    let start = d1.0.min(d2.0);
                    let end = d1.0.max(d2.0);
                    y1 < d1.1
                        && d1.1 < y2
                        && !((start <= x1 && end <= x1) || (start >= x2 && end >= x2))
                }
            }).is_none()
            /*coords
                .iter()
                .find(|(x, y)| x1 < *x && *x < x2 && y1 < *y && *y < y2)
                .is_none()
                */
        })
        .collect::<Vec<_>>();
    println!("{:?}", pairs);
    pairs.sort_by_key(|(_, _, area)| *area);
    println!("{:?}", pairs.last().ok_or("empty input")?);

    Ok(())
}
