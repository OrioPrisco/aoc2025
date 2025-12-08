use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

use vector::Vector3;

fn run(input: File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let lines: Vec<_> = input.lines().collect::<Result<_, _>>()?;
    let connections = if lines.len() > 50 { 1000 } else { 10 };
    let coords = lines
        .iter()
        .map(|s| Vector3::from_iterator(s.split(',').map(|s| s.parse::<i64>().unwrap())))
        .collect::<Vec<_>>();
    println!("{:?}", coords);

    let mut pairs = coords
        .iter()
        .enumerate()
        .map(|(i, c1)| {
            coords
                .iter()
                .skip(i + 1)
                .map(move |c2| (c1, c2, (c1 - c2).norm2()))
        })
        .flatten()
        .collect::<Vec<_>>();

    pairs.sort_by_key(|(_, _, dist)| *dist as u64);
    let pairs = pairs.iter().take(connections).collect::<Vec<_>>();
    for item in &pairs {
        println!("{:?}", item);
    }
    let mut networks = Vec::<HashSet<Vector3>>::new();
    for (&j1, &j2, _) in &pairs {
        let idx = networks
            .iter_mut()
            .position(|network| network.contains(&j1) || network.contains(&j2));
        match idx {
            None => {
                let mut set = HashSet::new();
                set.insert(j1);
                set.insert(j2);
                networks.push(set);
            }
            Some(_) => {
                let mut set = HashSet::new();
                for to_merge in networks.extract_if(.., |network| {
                    network.contains(&j1) || network.contains(&j2)
                }) {
                    set.extend(to_merge);
                }
                set.insert(j1);
                set.insert(j2);
                networks.push(set);
            }
        }
    }
    for item in &networks {
        println!("{} : {:?}", item.len(), item);
    }
    let mut sizes = networks.iter().map(HashSet::len).collect::<Vec<_>>();
    sizes.sort();
    println!("{}", sizes.iter().rev().take(3).product::<usize>());
    Ok(())
}

//from scop
pub mod vector {
    use std::ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
    };

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Vector3 {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }
    impl Vector3 {
        pub fn norm2(&self) -> i64 {
            self.x * self.x + self.y * self.y + self.z * self.z
        }
        pub fn zero() -> Vector3 {
            Self { x: 0, y: 0, z: 0 }
        }
        pub fn cross(&self, rhs: &Self) -> Self {
            Vector3 {
                x: self.y * rhs.z - self.z * rhs.y,
                y: self.z * rhs.x - self.x * rhs.z,
                z: self.x * rhs.y - self.y * rhs.x,
            }
        }
        pub fn from_iterator(mut it: impl Iterator<Item = i64>) -> Self {
            Self {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
            }
        }
    }
    macro_rules! vector3_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl $imp<i64> for Vector3 {
                type Output = Vector3;

                fn $method(self, rhs: i64) -> Self::Output {
                    Vector3 {
                        x: self.x $op rhs,
                        y: self.y $op rhs,
                        z: self.z $op rhs,
                    }
                }
            }
            impl $imp_assign<i64> for Vector3 {
                fn $method_assign(&mut self, rhs: i64) {
                    self.x $op_assign rhs;
                    self.y $op_assign rhs;
                    self.z $op_assign rhs;
                }
            }
        )*}
    }
    vector3_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
        Mul, mul, MulAssign, mul_assign, *, *=
        Div, div, DivAssign, div_assign, /, /=
    );
    macro_rules! vector3_self_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl  $imp for &Vector3 {
                type Output = Vector3;

                fn $method(self, rhs: Self) -> Self::Output {
                    Vector3 {
                        x: self.x $op rhs.x,
                        y: self.y $op rhs.y,
                        z: self.z $op rhs.z,
                    }
                }
            }
            impl $imp_assign for Vector3 {
                fn $method_assign(&mut self, rhs: Self) {
                    self.x $op_assign rhs.x;
                    self.y $op_assign rhs.y;
                    self.z $op_assign rhs.z;
                }
            }
        )*}
    }
    vector3_self_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
    );

    impl Neg for Vector3 {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
    impl Index<usize> for Vector3 {
        type Output = i64;
        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                x => panic!("Non existant index {}", x),
            }
        }
    }
    impl IndexMut<usize> for Vector3 {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                x => panic!("Non existant index {}", x),
            }
        }
    }
}
