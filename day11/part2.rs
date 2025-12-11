use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign};

fn main() {
    let input = File::open(env::args().nth(1).unwrap_or(String::from("input"))).unwrap();
    run(input).unwrap();
}

type NodeId = [u8; 3];

fn id_from_str(id: &str) -> NodeId {
    let id = id.as_bytes();
    assert!(id.len() == 3);
    id[0..3].try_into().unwrap()
}

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    outputs: Vec<NodeId>,
}

impl Node {
    fn from_string(string: &String) -> Self {
        let (id, outputs) = string.split_once(":").unwrap();
        Self {
            id: id_from_str(id),
            outputs: outputs.split_whitespace().map(id_from_str).collect(),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Costs {
    none : usize,
    dac : usize,
    fft : usize,
    both: usize,
}

impl Add for Costs {
    type Output=Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            none: self.none + rhs.none,
            dac: self.dac + rhs.dac,
            fft: self.fft + rhs.fft,
            both: self.both + rhs.both,
        }
    }
}

impl AddAssign for Costs {
    fn add_assign(&mut self, rhs: Self) {
        self.none = self.none + rhs.none;
        self.dac = self.dac + rhs.dac;
        self.fft = self.fft + rhs.fft;
        self.both = self.both + rhs.both;
    }
}

impl Costs {
    fn visit_dac(&mut self) {
        self.dac = self.dac + self.none;
        self.both = self.fft + self.both;
        self.none = 0;
        self.fft = 0;
    }
    fn visit_fft(&mut self) {
        self.fft = self.fft + self.none;
        self.both = self.dac + self.both;
        self.none = 0;
        self.dac = 0;
    }
}

//finds longest paths from every node
fn longest_path(nodes: &HashMap<NodeId, Node>) -> HashMap<NodeId, u64> {
    let mut rev_nodes = (*nodes).clone();
    for (_, val) in rev_nodes.iter_mut() {
        val.outputs.clear()
    }
    for (id, node) in nodes {
        for output in &node.outputs {
            rev_nodes.get_mut(output).unwrap().outputs.push(*id);
        }
    }

    let rev_nodes = rev_nodes;
    let mut paths = nodes.iter().map(|(id,_)| (*id, 0)).collect::<HashMap<_,_>>();
    let mut to_visit = VecDeque::<NodeId>::new();
    to_visit.push_back(id_from_str("out"));
    while let Some(id) = to_visit.pop_front() {
        let node = &rev_nodes[&id];
        let new_cost = paths[&id] + 1;
        for output in &node.outputs {
            let cost = paths[output];
            if new_cost > cost {
                *paths.get_mut(output).unwrap() = new_cost;
                to_visit.push_back(*output);
            }
        }
    }
    paths
}

fn find_paths(nodes: &HashMap<NodeId, Node>) {
    let paths = longest_path(nodes);
    let mut costs = nodes
        .iter()
        .map(|(id, _)| (*id, Costs::default()))
        .collect::<HashMap<_, _>>();
    costs.insert(id_from_str("svr"), Costs {
        none : 1,
        dac: 0,
        fft: 0,
        both: 0,
    });
    let mut visit_order = paths.iter().map(|(a,b)| (*a,*b)).collect::<Vec<(NodeId, u64)>>();
    visit_order.sort_by_key(|(_id,cost)| *cost);
    visit_order.reverse();
    for (id, _) in &visit_order {
        match id {
            [102,102,116] => {
                costs.get_mut(id).unwrap().visit_fft();
                println!("fft");
            }//fft
            [100,97,99] => costs.get_mut(id).unwrap().visit_dac(), //dac
            _ => (),
        }
        let node = &nodes[id];
        let cost = costs[id];
        for output in &node.outputs {
            *costs.get_mut(output).unwrap() += cost;
        }
    }
    //println!("{:?}", costs);
    println!("{:?}", costs[&id_from_str("out")]);
}

fn run(input: File) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(input);
    let lines: Vec<_> = input.lines().collect::<Result<_, _>>()?;
    let mut nodes = lines
        .iter()
        .map(Node::from_string)
        .map(|n| (n.id, n))
        .collect::<HashMap<_, _>>();
    let out = Node {
        id : id_from_str("out"),
        outputs : Vec::new(),
    };
    nodes.insert(out.id, out);
    find_paths(&nodes);

    Ok(())
}
