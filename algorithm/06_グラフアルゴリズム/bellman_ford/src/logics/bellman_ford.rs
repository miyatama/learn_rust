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
    let mut dist_costs = vec![u64::MAX; graph.len()];
    let mut nodes_from = vec![u64::MAX; graph.len()];
    let start_node_index = get_node_index(start_id, graph).unwrap();
    let terminate_node_index = get_node_index(terminate_id, graph).unwrap();
    dist_costs[start_node_index] = 0;

    for i in 0..graph.len() {
        let fail_on_update = i == (graph.len() - 1);
        let mut leave_early = true;

                eprintln!("checking node id: {}", graph[i].id);
        let edges = graph[i].edges.clone();

        // 辺を持たない節点は判断しない
        if edges.len() <= 0 {
            continue;
        }

        for edge_index in 0..edges.len() {
            let edge_node_id = edges[edge_index].node_id;
            let edge_node_index = get_node_index(edge_node_id, graph).unwrap();
            let new_cost = dist_costs[i] + edges[edge_index].cost;
            if new_cost < dist_costs[edge_node_index] {
                if fail_on_update {
                    return (None, None);
                }
                eprintln!("new path.{} to {}.cost: {}", graph[i].id,edge_node_id ,new_cost);
                dist_costs[edge_node_index] = new_cost;
                nodes_from[edge_node_index] = graph[i].id;
                leave_early = false;
            }
        }
        if leave_early {
            eprintln!("leave early");
            break;
        }
    }

    (
        generate_shortest_route(start_id, terminate_id, nodes_from, graph),
        Some(dist_costs[terminate_node_index]),
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
    from_ids: Vec<u64>,
    graph: &Vec<Node>,
) -> Option<Vec<Node>> {
    let mut node_id = terminate_id;
    let mut route = Vec::new();
    loop {
        eprintln!("shortest route node: {}", node_id);
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
