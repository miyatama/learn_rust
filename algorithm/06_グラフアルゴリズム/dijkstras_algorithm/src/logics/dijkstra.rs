use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u64,
    // node_index
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // cost -> positionの順で判定
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
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

/**
 * 最短経路を返す
 * nodes: [(id, [[(edge_id, cost], ...]), ...]
 */
pub fn search<W: Write>(
    w: &mut W,
    _n: u64,
    start_id: u64,
    terminate_id: u64,
    nodes: Vec<(u64, Vec<(u64, u64)>)>,
) {
    let graph = parse_to_graph(&nodes);
    print_graph(&graph);
    let (route, cost) = shortest_path_search(start_id, terminate_id, &graph);
    match route {
        None => writeln!(w, "route not found").unwrap(),
        Some(route) => {
            eprintln!("route: {:?}", generate_route_text(&route));
            eprintln!("cost: {}", cost.unwrap());
            route.iter().for_each(|val| {
                writeln!(w, "{}", val.id).unwrap();
            });
        }
    }
}

fn shortest_path_search(
    start_id: u64,
    terminate_id: u64,
    graph: &Vec<Node>,
) -> (Option<Vec<Node>>, Option<u64>) {
    eprintln!("shortest path search: {} to {}", start_id, terminate_id);
    let mut dist_cost = vec![u64::MAX; graph.len()];
    let mut node_from = graph.iter().map(|node| node.id).collect::<Vec<_>>();
    let mut pq = BinaryHeap::new();

    let start_node_index = get_node_index(start_id, &graph).unwrap();
    let terminate_node_index = get_node_index(terminate_id, &graph).unwrap();
    dist_cost[start_node_index] = 0;
    pq.push(State {
        cost: 0,
        position: start_node_index,
    });
    while let Some(State { cost, position }) = pq.pop() {
        if position == terminate_node_index {
            return (
                generate_shortest_route(start_id, terminate_id, node_from, graph),
                Some(dist_cost[terminate_node_index]),
            );
        }
        if cost > dist_cost[position] {
            continue;
        }

        let edges = graph[position].edges.clone();
        for i in 0..edges.len() {
            let next_node_index = get_node_index(edges[i].node_id, &graph).unwrap();
            let next = State {
                cost: cost + edges[i].cost,
                position: next_node_index,
            };
            if next.cost < dist_cost[next.position] {
                pq.push(next);
                dist_cost[next.position] = next.cost;
                eprintln!(
                    "set {} node from: {}",
                    graph[next.position].id, graph[position].id
                );
                node_from[next.position] = graph[position].id;
            }
        }
    }
    (None, None)
}

fn generate_shortest_route(
    start_id: u64,
    terminate_id: u64,
    from_ids: Vec<u64>,
    graph: &Vec<Node>,
) -> Option<Vec<Node>> {
    let mut node_id = terminate_id;
    let mut route = Vec::new();
    loop {
        let node_index = get_node_index(node_id, graph).unwrap();
        route.push(graph[node_index].clone());
        if node_id == start_id {
            break;
        }
        node_id = from_ids[node_index];
    }
    Some(route.into_iter().rev().collect::<Vec<_>>().to_vec())
}

fn generate_route_text(route: &Vec<Node>) -> String {
    route
        .iter()
        .map(|node| node.id.to_string())
        .collect::<Vec<String>>()
        .join(" -> ")
}

fn print_graph(nodes: &Vec<Node>) {
    for i in 0..nodes.len() {
        eprintln!(
            "id: {}, edge: {:?}",
            nodes[i].id,
            nodes[i]
                .edges
                .iter()
                .map(|edge| edge.node_id)
                .collect::<Vec<_>>()
        );
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
