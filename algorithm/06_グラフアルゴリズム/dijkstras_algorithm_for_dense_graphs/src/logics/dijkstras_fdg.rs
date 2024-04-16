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
    let mut node_from = vec![start_id; graph.len()];
    let mut visiteds = vec![false; graph.len()];
    let start_index = get_node_index(start_id, graph).unwrap();
    let terminate_index = get_node_index(terminate_id, graph).unwrap();

    dist_costs[start_index] = 0;
    loop {
        eprintln!("visiteds: {:?}", visiteds);
        eprintln!("costs: {:?}", dist_costs);
        if visiteds
            .iter()
            .zip(dist_costs.iter())
            .map(|(visited, cost)| (*visited, *cost))
            .filter(|(visited, dist_cost)| *visited == false && *dist_cost < u64::MAX)
            .count() <= 0
        {
            eprintln!("finish to searching");
            break;
        }
        let index = visiteds
            .iter()
            .zip(dist_costs.iter())
            .enumerate()
            .filter(|(_, (visited, _))| **visited == false)
            .map(|(index, (_, cost))| (index, cost))
            .fold((usize::MIN, u64::MAX), |(aindex, a), (bindex, b)| {
                if a < *b {
                    (aindex, a)
                } else {
                    (bindex, *b)
                }
            }).0;
        eprintln!("target node: {}", graph[index].id);
        if dist_costs[index] == u64::MAX {
            break;
        }
        visiteds[index] = true;
        let edges = graph[index].edges.clone();
        for i in 0..edges.len() {
            let edge_index = get_node_index(edges[i].node_id, graph).unwrap();
            eprintln!("edge node id: {}, index: {}", edges[i].node_id, edge_index);
            let new_cost = edges[i].cost + edges[i].cost;
            if new_cost < dist_costs[edge_index] {
                eprintln!("new route! {} -> {}, new cost: {}", graph[index].id, edges[i].node_id, new_cost);
                dist_costs[edge_index] = new_cost;
                node_from[index] = edges[i].node_id;
            }
        }
    }

    (
        generate_shortest_route(start_id, terminate_id, node_from, graph),
        Some(dist_costs[terminate_index]),
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
        let expect = vec!["0", "3", "2", "5"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
