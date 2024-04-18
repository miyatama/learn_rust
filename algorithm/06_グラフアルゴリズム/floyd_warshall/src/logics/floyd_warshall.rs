use std::io::Write;

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
    let mut dist_costs = vec![vec![u64::MAX; graph.len()]; graph.len()];
    let mut node_from = vec![vec![start_id; graph.len()]; graph.len()];
    let start_index = get_node_index(start_id, graph).unwrap();
    let terminate_index = get_node_index(terminate_id, graph).unwrap();
    dist_costs[start_index][start_index] = 0;
    for node_index in 0..graph.len() {
        let node_id = graph[node_index].id;
        let edges = graph[node_index].edges.clone();
        for j in 0..edges.len() {
            let edge_index = get_node_index(edges[j].node_id, graph).unwrap();
            dist_costs[node_index][edge_index] = edges[j].cost;
            node_from[node_index][edge_index] = node_id;
        }
    }

    for relay_node_index in 0..graph.len() {
        let relay_node_id = graph[relay_node_index].id;
        for start_node_index in 0..graph.len() {
            let start_node_id = graph[start_node_index].id;
            eprintln!("check cost path: {} to {}", start_node_id, relay_node_id);
            if dist_costs[start_node_index][relay_node_index] == u64::MAX {
                continue;
            }

            for terminate_node_index in 0..graph.len() {
                let terminate_node_id = graph[terminate_node_index].id;
                eprintln!(
                    "check cost path: {} to {} to {}",
                    start_node_id, relay_node_id, terminate_node_id
                );
                if dist_costs[relay_node_index][terminate_node_index] == u64::MAX {
                    continue;
                }
                let new_cost = dist_costs[start_node_index][relay_node_index]
                    + dist_costs[relay_node_index][terminate_node_index];
                if new_cost < dist_costs[start_node_index][terminate_node_index] {
                    eprintln!(
                        "new path: {} to {}.cost: {}",
                        start_node_id, terminate_node_id, new_cost
                    );
                    dist_costs[start_node_index][terminate_node_index] = new_cost;
                    node_from[start_node_index][terminate_node_index] =
                        node_from[relay_node_index][terminate_node_index];
                }
            }
        }
    }
    eprintln!("calculated costs");

    (
        generate_shortest_route(start_id, terminate_id, node_from, graph),
        Some(dist_costs[start_index][terminate_index]),
    )
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

fn generate_shortest_route(
    start_id: u64,
    terminate_id: u64,
    from_ids: Vec<Vec<u64>>,
    graph: &Vec<Node>,
) -> Option<Vec<Node>> {
    let start_node_index = get_node_index(start_id, graph).unwrap();
    let mut node_id = terminate_id;
    let mut route = Vec::new();
    loop {
        eprintln!("path id: {}", node_id);
        let node_index = get_node_index(node_id, graph).unwrap();
        route.push(graph[node_index].clone());
        if node_id == start_id {
            break;
        }
        node_id = from_ids[start_node_index][node_index];
        eprintln!("new path id: {}", node_id);
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
