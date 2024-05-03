use super::graph::{EdgeDirection, Graph, VertexInfo, VertexType};
use std::collections::{HashMap, VecDeque};

pub fn maximum_flow(graph: &Graph) -> Graph {
    let mut graph = graph.clone();
    while let Some(vertex_info) = find_argumenting_path(&graph) {
        graph = process_path(&graph, &vertex_info);
    }
    graph
}

fn find_argumenting_path(graph: &Graph) -> Option<HashMap<u32, VertexInfo>> {
    let shink_vertex = graph.get_shink_vertex();
    let mut path: HashMap<u32, VertexInfo> = HashMap::new();
    let mut queue: VecDeque<u32> = VecDeque::new();
    let source_vertex = graph.get_source_vertex();
    queue.push_back(source_vertex.id);
    while let Some(from_id) = queue.pop_back() {
        let edges = graph.get_forward_edges(from_id);
        for i in 0..edges.len() {
            let edge = &edges[i];
            if !path.contains_key(&edge.to_id) && edge.capacity > edge.flow {
                path.insert(
                    edge.to_id,
                    VertexInfo {
                        previous: from_id,
                        direction: EdgeDirection::Forward,
                    },
                );
                if edge.to_id == shink_vertex.id {
                    return Some(path);
                }
                queue.push_back(edge.to_id);
            }
        }
        let edges = graph.get_backward_edges(from_id);
        for i in 0..edges.len() {
            let edge = &edges[i];
            if !path.contains_key(&edge.from_id) && edge.flow > 0 {
                path.insert(
                    edge.from_id,
                    VertexInfo {
                        previous: from_id,
                        direction: EdgeDirection::Backward,
                    },
                );
                queue.push_back(edge.from_id);
            }
        }
    }
    None
}

fn process_path(graph: &Graph, path: &HashMap<u32, VertexInfo>) -> Graph {
    let mut graph = graph.clone();
    let mut vertex = graph.get_shink_vertex();
    let mut delta = i64::MAX;

    while vertex.vertex_type != VertexType::Source {
        let vertex_info = path.get(&vertex.id).unwrap();
        let previous = vertex_info.previous;
        let flow = match vertex_info.direction {
            EdgeDirection::Forward => {
                let edge = graph.get_edge(previous, vertex.id);
                edge.capacity - edge.flow
            }
            EdgeDirection::Backward => {
                let edge = graph.get_edge(previous, vertex.id);
                edge.flow
            }
        };
        if flow < delta {
            delta = flow;
        }
        vertex = graph.get_vertex(previous);
    }

    // apply delta
    vertex = graph.get_shink_vertex();
    while vertex.vertex_type != VertexType::Source {
        let vertex_info = path.get(&vertex.id).unwrap();
        let previous = vertex_info.previous;
        match vertex_info.direction {
            EdgeDirection::Forward => {
                graph = graph.apply_flow(previous, vertex.id, delta);
            }
            EdgeDirection::Backward => {
                graph = graph.apply_flow(previous, vertex.id, -delta);
            }
        }
        vertex = graph.get_vertex(previous);
    }
    graph
}
