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
    let map = Map::from_lines(input.iter().map(|s| s.as_bytes()));
    println!("{map:?}");
    let count = map.iter().filter(|(_, _, &c)| c == '@' as u8).map(|(x, y, _)|
        map.full_around(x,y).filter(|(_, _, &c)| c == '@' as u8).count()
    ).collect::<Vec<_>>();
    println!("{:?}", count);
    let count = count.iter().filter(|&count| *count < 4).count();
    println!("{}", count);
    Ok(())
}

fn try_add_dir<T>(map: &Map<T>, (x, y): (u16, u16), (dir_x, dir_y): (i16, i16)) -> Option<(u16, u16)> {
    if x == 0 && dir_x < 0 {
        return None;
    }
    if y == 0 && dir_y < 0 {
        return None;
    }
    if x + dir_x as u16 == map.width() {
        return None;
    }
    if y + dir_y as u16 == map.height() {
        return None;
    }
    Some((x + dir_x as u16, y + dir_y as u16))
}

#[rustfmt::skip]
pub const AROUND_DIRS : [(i16,i16);8] = [
    (-1, -1), ( 0, -1), ( 1,-1),
    (-1,  0),           ( 1, 0), // had to remove center here
    (-1,  1), ( 0,  1), ( 1, 1),
];

#[rustfmt::skip]
pub const CROSS_DIRS : [(i16,i16);5] = [
              ( 0, -1),
    (-1,  0), ( 0,  0), ( 1, 0),
              ( 0,  1),
];

#[derive(Debug, Clone)]
pub struct Map<T> {
    cells: Box<[T]>,
    width: u16,
    height: u16,
}

impl<T: Clone> Map<T> {
    pub fn new(width: u16, height: u16, default: T) -> Self {
        let mut vec: Vec<T> = Vec::with_capacity((width * height) as usize);
        vec.resize((height * width) as usize, default);
        Map {
            cells: vec.into_boxed_slice(),
            width,
            height,
        }
    }
    pub fn from_lines<'a>(lines: impl Iterator<Item=&'a[T]> + Clone) -> Self where T: 'a {
        let mut vec: Vec<T> = Vec::new();
        let width = lines.clone().next().unwrap().len();
        let height = lines.clone().count();
        for line in lines {
            assert!(line.len() == width);
            vec.extend_from_slice(line);
        }
        Map {
            cells: vec.into_boxed_slice(),
            width: width as u16,
            height: height as u16,
        }
    }
}

impl<T> Map<T> {
    pub fn at(&self, x: u16, y: u16) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.cells[(y * self.width + x) as usize]
    }
    pub fn at_mut(&mut self, x: u16, y: u16) -> &mut T {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.cells[(y * self.width + x) as usize]
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn iter<'a>(&'a self) -> MapIter<'a, T> {
        MapIter {
            iter: self.cells.iter(),
            x: 0,
            width: self.width,
        }
    }
    pub fn raw_iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> MapIterMut<'a, T> {
        MapIterMut {
            iter: self.cells.iter_mut(),
            x: 0,
            width: self.width,
        }
    }
    pub fn around<'a>(&'a self, x: u16, y: u16) -> MapAroundIter<'a, T> {
        assert!(x < self.width);
        assert!(y < self.height);
        MapAroundIter {
            map: self,
            x,
            y,
            i: 0,
        }
    }
    pub fn full_around<'a>(&'a self, x: u16, y: u16) -> MapFullAroundIter<'a, T> {
        assert!(x < self.width);
        assert!(y < self.height);
        MapFullAroundIter {
            map: self,
            x,
            y,
            i: 0,
        }
    }
}

pub struct MapAroundIter<'a, T> {
    map: &'a Map<T>,
    x: u16,
    y: u16,
    i: i16,
}

impl<'a, T> Iterator for MapAroundIter<'a, T> {
    type Item = (u16, u16, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        if self.i == 0 {
            self.i += 1;
            if x > 0 {
                return Some((x - 1, y, self.map.at(x - 1, y)));
            }
        }
        if self.i == 1 {
            self.i += 1;
            if y > 0 {
                return Some((x, y - 1, self.map.at(x, y - 1)));
            }
        }
        if self.i == 2 {
            self.i += 1;
            if x != self.map.width - 1 {
                return Some((x + 1, y, self.map.at(x + 1, y)));
            }
        }
        if self.i == 3 {
            self.i += 1;
            if y != self.map.height - 1 {
                return Some((x, y + 1, self.map.at(x, y + 1)));
            }
        }
        if self.i == 4 {
            self.i += 1;
            if y != self.map.height - 1 {
                return Some((x, y, self.map.at(x, y)));
            }
        }
        None
    }
}

pub struct MapFullAroundIter<'a, T> {
    map: &'a Map<T>,
    x: u16,
    y: u16,
    i: u16,
}

impl<'a, T> Iterator for MapFullAroundIter<'a, T> {
    type Item = (u16, u16, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        while (self.i as usize) < AROUND_DIRS.len() {
            let i = self.i;
            self.i += 1;
            if let Some((x, y)) = try_add_dir(self.map, (x, y), AROUND_DIRS[i as usize]) {
                return Some((x, y, self.map.at(x, y)));
            }
        }
        None
    }
}

pub struct MapIterMut<'a, T> {
    iter: std::slice::IterMut<'a, T>,
    x: u16,
    width: u16,
}

impl<'a, T> Iterator for MapIterMut<'a, T> {
    type Item = (u16, u16, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        self.x += 1;
        self.iter
            .next()
            .map(|i| (x % self.width, x / self.width, i))
    }
}

pub struct MapIter<'a, T> {
    iter: std::slice::Iter<'a, T>,
    x: u16,
    width: u16,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = (u16, u16, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        self.x += 1;
        self.iter
            .next()
            .map(|i| (x % self.width, x / self.width, i))
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type Item = (u16, u16, &'a T);
    type IntoIter = MapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Map<T> {
    type Item = (u16, u16, &'a mut T);
    type IntoIter = MapIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

