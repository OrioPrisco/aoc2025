use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::error::Error;

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

fn tachyon(field : &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    if y == field.len() {
        return 0;
    }
    match field[y][x] as char {
        'S' => tachyon(field, x, y+1),
        '^' => 1 + tachyon(field, x+1, y) + tachyon(field, x-1, y),
        '.' => {
            field[y][x] = '|' as u8;
            tachyon(field, x, y+1)
        }
        '|' => 0,
        x => panic!("Unknown item {}", x),
    }

}

fn run(input : File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let mut lines : Vec<_> = input.lines().map(|s| s.map(String::into_bytes)).collect::<Result<_,_>>()?;

    let starting_pos = lines.iter().enumerate().find_map(|(i,s)| s.iter().position(|&c| c == 'S' as u8).map(|p| (i,p))).unwrap();
    let (y,x) = starting_pos;
    println!("{x}, {y}");
    let res = tachyon(&mut lines, x, y);
    for line in lines {
        println!("{:?}", str::from_utf8(&line));
    }
    println!("{res}");
    Ok(())
}
