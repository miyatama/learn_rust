use super::graph::{EdgeDirection, Graph, VertexInfo, VertexType};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    vertex_id: u32,
    priority: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // priorityで判定
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn minimal_cost_flow(graph: &Graph) -> Graph {
    let mut graph = graph.clone();
    while let Some(vertex_info) = find_argumenting_path(&graph) {
        print_path(&graph, &vertex_info);
        graph = process_path(&graph, &vertex_info);
    }
    graph
}

fn find_argumenting_path(graph: &Graph) -> Option<HashMap<u32, VertexInfo>> {
    let source_vertex = graph.get_source_vertex();
    let shink_vertex = graph.get_shink_vertex();
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    let mut arrival_cost: HashMap<u32, i64> = HashMap::new();
    let mut path: HashMap<u32, VertexInfo> = HashMap::new();
    pq.push(State {
        vertex_id: source_vertex.id,
        priority: 0,
    });
    arrival_cost.insert(source_vertex.id, 0i64);

    while let Some(state) = pq.pop() {
        let from_vertex = graph.get_vertex(state.vertex_id);
        if from_vertex.id == shink_vertex.id {
            return Some(path);
        }
        for vertex_index in 0..graph.vertexes.len() {
            let to_vertex = &graph.vertexes[vertex_index];
            if to_vertex.id == source_vertex.id || to_vertex.id == from_vertex.id {
                continue;
            }
            let edge = graph.get_forward_edge(from_vertex.id, to_vertex.id);
            if edge.is_some() {
                let edge = edge.unwrap();
                if edge.flow < edge.capacity {
                    let new_cost =
                        add_cost(get_arrival_cost(&arrival_cost, from_vertex.id), edge.cost);
                    if 0 <= new_cost && new_cost < get_arrival_cost(&arrival_cost, to_vertex.id) {
                        path.insert(
                            edge.to_id,
                            VertexInfo {
                                previous: from_vertex.id,
                                direction: EdgeDirection::Forward,
                            },
                        );
                        arrival_cost.insert(to_vertex.id, new_cost);
                        pq = BinaryHeap::from(
                            pq.into_vec()
                                .into_iter()
                                .filter(|state| state.vertex_id != to_vertex.id)
                                .collect::<Vec<State>>(),
                        );
                        pq.push(State {
                            vertex_id: to_vertex.id,
                            priority: new_cost,
                        });
                    }
                }
            }

            // Backwards
            let edge = graph.get_backward_edge(to_vertex.id, from_vertex.id);
            if edge.is_some() {
                let edge = edge.unwrap();
                if edge.flow > 0 {
                    let new_cost = add_cost(
                        get_arrival_cost(&arrival_cost, from_vertex.id),
                        -edge.get_cost(),
                    );
                    if 0 <= new_cost && new_cost < get_arrival_cost(&arrival_cost, to_vertex.id) {
                        path.insert(
                            edge.to_id,
                            VertexInfo {
                                previous: from_vertex.id,
                                direction: EdgeDirection::Backward,
                            },
                        );
                        arrival_cost.insert(to_vertex.id, new_cost);
                        pq = BinaryHeap::from(
                            pq.into_vec()
                                .into_iter()
                                .filter(|state| state.vertex_id != to_vertex.id)
                                .collect::<Vec<State>>(),
                        );
                        pq.push(State {
                            vertex_id: to_vertex.id,
                            priority: new_cost,
                        });
                    }
                }
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
                let edge = graph.get_edge(vertex.id, previous);
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
                graph = graph.apply_flow(vertex.id, previous, -delta);
            }
        }
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

fn get_arrival_cost(costs: &HashMap<u32, i64>, id: u32) -> i64 {
    *costs.get(&id).unwrap_or(&i64::MAX)
}

fn print_path(graph: &Graph, path: &HashMap<u32, VertexInfo>) {
    let mut path_texts: Vec<String> = Vec::new();
    let mut vertex = graph.get_shink_vertex();
    while vertex.vertex_type != VertexType::Source {
        let vertex_info = path.get(&vertex.id).unwrap();
        let previous = vertex_info.previous;
        match vertex_info.direction {
            EdgeDirection::Forward => {
                path_texts.push(format!("{}({})", vertex.id, "F"));
            }
            EdgeDirection::Backward => {
                path_texts.push(format!("{}({})", vertex.id, "B"));
            }
        };
        vertex = graph.get_vertex(previous);
    }
    path_texts.push(format!("{}", vertex.id));
    println!("{}", path_texts.join(" -> "));
}
