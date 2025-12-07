use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::error::Error;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn tachyon(field : &mut Vec<Vec<u8>>, cache: &mut Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
    if y == field.len() {
        return 1;
    }
    match field[y][x] as char {
        'S' => tachyon(field, cache, x, y+1),
        '|' => cache[y][x],
        '^' => tachyon(field, cache, x+1, y) + tachyon(field, cache, x-1, y),
        '.' => {
            field[y][x] = '|' as u8;
            let res = tachyon(field, cache, x, y+1);
            cache[y][x] = res;
            res
        }
        x => panic!("Unknown item {}", x),
    }

}

fn run(input : File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let mut lines : Vec<_> = input.lines().map(|s| s.map(String::into_bytes)).collect::<Result<_,_>>()?;

    let starting_pos = lines.iter().enumerate().find_map(|(i,s)| s.iter().position(|&c| c == 'S' as u8).map(|p| (i,p))).unwrap();
    let (y,x) = starting_pos;
    println!("{x}, {y}");
    let mut cache = lines.iter().map(|line| line.iter().map(|_| 0).collect::<Vec<u64>>()).collect::<Vec<_>>();
    let res = tachyon(&mut lines, &mut cache, x, y);
    for line in lines {
        println!("{:?}", str::from_utf8(&line));
    }
    println!("{res}");
    Ok(())
}
