use daggy::{Dag, NodeIndex, Walker};
use std::collections::{HashMap, HashSet};

type IndexMap = HashMap<String, NodeIndex>;
type BagDag = Dag<String, u32>;


const INPUT: &str = include_str!("../input");
lazy_static::lazy_static! {
    static ref LINE_REGEX: regex::Regex = regex::Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

#[derive(Debug)]
struct Bags {
    indices: IndexMap,
    dag: BagDag,
}

fn containers(idx: NodeIndex, dag: &BagDag) -> HashSet<NodeIndex> {
    let mut set = HashSet::new();
    let parent_iter = dag.parents(idx);
    for (_edge, node) in parent_iter.iter(&dag) {
        set.insert(node);
        set.extend(containers(node, &dag));
    }
    set
}

fn containees_count(idx: NodeIndex, dag: &BagDag) -> u32 {
    let children = dag.children(idx);
    let mut count = 0;
    for (e, n) in children.iter(&dag) {
        let number_of_bags = *dag.edge_weight(e).unwrap();
        count += number_of_bags + (number_of_bags * containees_count(n, &dag));
    }
    count
}

fn parse_line(line: &str) -> (&str, Vec<(u32, &str)>) {
    let name_len = line
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()
        .join(" ")
        .len();

    let mut out = Vec::new();
    for capture in LINE_REGEX.captures_iter(line) {
        let count = capture.get(1).unwrap().as_str().parse().unwrap();
        let name = capture.get(2).unwrap().as_str();
        out.push((count, name));
    }

    (&line[0..name_len], out)
}

fn populate_bagdag() -> Bags {
    let mut indices = HashMap::new();
    let mut dag: BagDag = Dag::new();

    for line in INPUT.lines() {
        let (name, children) = parse_line(&line);
        let parent_name = name.to_string();

        let parent_index = *indices
            .entry(parent_name)
            .or_insert_with(|| dag.add_node(name.to_string()));

        for (count, child_name) in children {
            let name = child_name.to_string();
            let child_index = indices
                .entry(name)
                .or_insert_with(|| dag.add_node(child_name.to_string()));
            dag.add_edge(parent_index, *child_index, count).unwrap();
        }
    }

    Bags { indices, dag }
}

fn main() {
    let bags = populate_bagdag();
    let idx = bags.indices.get("shiny gold").unwrap();
    let part1 = containers(*idx, &bags.dag).len();
    let part2 = containees_count(*idx, &bags.dag);

    println!("part1: {:?}, part2: {:?}", part1, part2);
}
