use super::graph::{EdgeDirection, Graph, VertexInfo, VertexType};
use std::collections::{HashMap, VecDeque};

pub fn transshipment(graph: &Graph) -> Graph {
    let mut graph = graph.clone();
    while let Some(vertex_info) = find_argumenting_path(&graph) {
        graph = process_path(&graph, &vertex_info);
    }
    graph
}

fn find_argumenting_path(graph: &Graph) -> Option<HashMap<usize, VertexInfo>> {
    let shink_vertex_index = graph.vertexes.len() - 1;
    let mut path: HashMap<usize, VertexInfo> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut arrival_cost: HashMap<usize, i64> = HashMap::new();
    // source indexは必ず1
    let source_vertex_index = 0usize;
    queue.push_front(source_vertex_index);
    arrival_cost.insert(source_vertex_index, 0i64);

    while let Some(from_vertex_index) = queue.pop_front() {
        let edges = graph.get_forward_edges(from_vertex_index);
        for i in 0..edges.len() {
            let edge = &edges[i];
            let new_cost = add_cost(
                get_arrival_cost(&arrival_cost, from_vertex_index),
                edge.cost,
            );
            if !path.contains_key(&edge.to_vertex_index)
                && edge.capacity > edge.flow
                && 0 <= new_cost
                && new_cost < get_arrival_cost(&arrival_cost, edge.to_vertex_index)
            {
                path.insert(
                    edge.to_vertex_index,
                    VertexInfo {
                        previous: from_vertex_index,
                        direction: EdgeDirection::Forward,
                    },
                );
                if edge.to_vertex_index == shink_vertex_index {
                    return Some(path);
                }
                arrival_cost.insert(edge.to_vertex_index, new_cost);
                queue.push_back(edge.to_vertex_index);
            }
        }
        let edges = graph.get_backward_edges(from_vertex_index);
        for i in 0..edges.len() {
            let edge = &edges[i];
            let new_cost = add_cost(
                get_arrival_cost(&arrival_cost, from_vertex_index),
                -edge.get_cost(),
            );
            if !path.contains_key(&edge.from_vertex_index)
                && edge.flow > 0
                && 0 <= new_cost
                && new_cost < get_arrival_cost(&arrival_cost, edge.to_vertex_index)
            {
                path.insert(
                    edge.from_vertex_index,
                    VertexInfo {
                        previous: from_vertex_index,
                        direction: EdgeDirection::Backward,
                    },
                );
                arrival_cost.insert(edge.to_vertex_index, new_cost);
                queue.push_back(edge.from_vertex_index);
            }
        }
    }
    None
}

fn process_path(graph: &Graph, path: &HashMap<usize, VertexInfo>) -> Graph {
    let mut graph = graph.clone();
    let mut vertex_index = graph.vertexes.len() - 1;
    let mut vertex = graph.get_shink_vertex();
    let mut delta = i64::MAX;

    while vertex.vertex_type != VertexType::Source {
        let vertex_info = path.get(&vertex_index).unwrap();
        let previous = vertex_info.previous;
        let flow = match vertex_info.direction {
            EdgeDirection::Forward => {
                let edge = graph.get_edge(previous, vertex_index);
                edge.capacity - edge.flow
            }
            EdgeDirection::Backward => {
                let edge = graph.get_edge(vertex_index, previous);
                edge.flow
            }
        };
        if flow < delta {
            delta = flow;
        }
        vertex_index = previous;
        vertex = graph.get_vertex(previous);
    }

    // apply delta
    vertex_index = graph.vertexes.len() - 1;
    vertex = graph.get_shink_vertex();
    while vertex.vertex_type != VertexType::Source {
        let vertex_info = path.get(&vertex_index).unwrap();
        let previous = vertex_info.previous;
        match vertex_info.direction {
            EdgeDirection::Forward => {
                graph = graph.apply_flow(previous, vertex_index, delta);
            }
            EdgeDirection::Backward => {
                graph = graph.apply_flow(vertex_index, previous, -delta);
            }
        }
        vertex_index = previous;
        vertex = graph.get_vertex(previous);
    }
    graph
}

fn add_cost(a: i64, b: i64) -> i64 {
    if a == i64::MAX || b == i64::MAX {
        i64::MAX
    } else {
        a + b
    }
}

fn get_arrival_cost(costs: &HashMap<usize, i64>, id: usize) -> i64 {
    *costs.get(&id).unwrap_or(&i64::MAX)
}
