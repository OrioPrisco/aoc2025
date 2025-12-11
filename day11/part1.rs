use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug)]
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

//probably could bfs to find all dists to process nodes in optimal order
fn find_paths(nodes: &HashMap<NodeId, Node>) {
    let mut to_visit = VecDeque::<(usize, NodeId)>::new();
    let mut costs = nodes
        .iter()
        .map(|(id, _)| (id, 0))
        .collect::<HashMap<_, _>>();
    to_visit.push_back((1, id_from_str("you")));
    while let Some((cost, id)) = to_visit.pop_front() {
        let node = &nodes[&id];
        for id in &node.outputs {
            *costs.get_mut(dbg!(id)).unwrap() += cost;
        }
        to_visit.extend(node.outputs.iter().map(|id| (cost, *id)));
    }
    println!("{}", costs[&id_from_str("out")]);
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
