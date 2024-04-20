use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u64,
    node_id: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // cost -> node_idの順で判定
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    node_id: u64,
    cost: u64,
}

#[derive(Clone, Debug)]
struct Node {
    id: u64,
    edges: Vec<Edge>,
}

pub fn search<W: Write>(w: &mut W, nodes: Vec<(u64, Vec<(u64, u64)>)>) {
    let graph = parse_to_graph(&nodes);
    print_graph(&graph);
    let mst = compute_mst(&graph);
    match mst {
        None => writeln!(w, "mst not found").unwrap(),
        Some(route) => {
            writeln!(w, "---").unwrap();
            writeln!(w, "title: MST").unwrap();
            writeln!(w, "---").unwrap();
            writeln!(w, "flowchart LR").unwrap();
            for i in 0..graph.len() {
                let node_id = graph[i].id;
                let from_node_id = route[i];
                if node_id == from_node_id {
                    continue;
                }
                writeln!(
                    w,
                    "  id{}:(({})) --- id{}:(({}))",
                    from_node_id, from_node_id, node_id, node_id
                )
                .unwrap();
            }
        }
    }
}

fn compute_mst(graph: &Vec<Node>) -> Option<Vec<u64>> {
    let mut pq = BinaryHeap::new();
    let mut keys = vec![u64::MAX; graph.len()];
    let mut node_from = vec![0; graph.len()];
    keys[0] = 0;
    for i in 0..graph.len() {
        let node_id = graph[i].id;
        pq.push(State {
            cost: keys[i],
            node_id: node_id,
        });
    }
    while let Some(State { cost: _, node_id }) = pq.pop() {
        let node_index = get_node_index(node_id, graph).unwrap();
        let edges = graph[node_index].edges.clone();
        for i in 0..edges.len() {
            let edge_node_id = edges[i].node_id;
            let edge_cost = edges[i].cost;
            let edge_node_index = get_node_index(edge_node_id, graph).unwrap();
            if pq
                .iter()
                .filter(|state| state.node_id == edge_node_id)
                .next()
                .is_none()
            {
                continue;
            }
            if edge_cost >= keys[edge_node_index] {
                continue;
            }
            node_from[edge_node_index] = node_id;
            keys[edge_node_index] = edge_cost;
            let mut pqv = pq.into_vec();
            for i in 0..pqv.len() {
                if pqv[i].node_id == edge_node_id {
                    pqv[i].cost = edge_cost;
                }
            }
            pq = pqv.into();
        }
    }
    Some(node_from)
}

fn print_graph(graph: &Vec<Node>) {
    eprintln!("---");
    eprintln!("title: graph");
    eprintln!("---");
    eprintln!("flowchart LR");
    for i in 0..graph.len() {
        let node_id = graph[i].id;
        let edges = graph[i].edges.clone();
        for j in 0..edges.len() {
            let cost = edges[j].cost;
            let edge_node_id = edges[j].node_id;
            eprintln!(
                "  id{}:(({}))-- {} ---id{}:(({}))",
                node_id, node_id, cost, edge_node_id, edge_node_id
            );
        }
    }
}

fn get_node_index(id: u64, nodes: &Vec<Node>) -> Option<usize> {
    let index = nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.id == id)
        .next()
        .map(|(i, _)| i);
    if index.is_none() {
        eprintln!("node not found in graph.id: {}", id);
    }
    index
}

fn parse_to_graph(nodes: &Vec<(u64, Vec<(u64, u64)>)>) -> Vec<Node> {
    nodes
        .iter()
        .map(|node| Node {
            id: node.0,
            edges: node
                .1
                .iter()
                .map(|edge| Edge {
                    node_id: edge.0,
                    cost: edge.1,
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_01() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            6,
            0,
            5,
            vec![
                (0, vec![(1, 6), (2, 8), (3, 18)]),
                (1, vec![(4, 11)]),
                (2, vec![(3, 9)]),
                (3, vec![]),
                (4, vec![(5, 3)]),
                (5, vec![(3, 4), (2, 7)]),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0", "1", "4", "5"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
