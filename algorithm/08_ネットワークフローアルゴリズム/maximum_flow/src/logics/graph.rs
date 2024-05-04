#[derive(Debug, Clone, PartialEq)]
pub enum VertexType {
    None,
    Shink,
    Source,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: u32,
    pub name: String,
    pub vertex_type: VertexType,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub from_id: u32,
    pub to_id: u32,
    pub flow: i64,
    pub capacity: i64,
}

impl Edge {
    pub fn add_flow(&self, delta: i64) -> Edge {
        Edge {
            from_id: self.from_id,
            to_id: self.to_id,
            flow: self.flow + delta,
            capacity: self.capacity,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn get_vertex(&self, id: u32) -> Vertex {
        self.vertexes
            .clone()
            .into_iter()
            .filter(|vertex| vertex.id == id)
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_edge(&self, from_id: u32, to_id: u32) -> Edge {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_id == from_id && edge.to_id == to_id)
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_shink_vertex(&self) -> Vertex {
        self.vertexes
            .clone()
            .into_iter()
            .filter(|vertex| vertex.vertex_type == VertexType::Shink)
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_source_vertex(&self) -> Vertex {
        self.vertexes
            .clone()
            .into_iter()
            .filter(|vertex| vertex.vertex_type == VertexType::Source)
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_forward_edges(&self, vertex_id: u32) -> Vec<Edge> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_id == vertex_id)
            .collect::<Vec<Edge>>()
            .clone()
    }

    pub fn get_backward_edges(&self, vertex_id: u32) -> Vec<Edge> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.to_id == vertex_id)
            .collect::<Vec<Edge>>()
            .clone()
    }

    pub fn apply_flow(&self, from_id: u32, to_id: u32, delta: i64) -> Graph {
        let edge = self.get_edge(from_id, to_id).add_flow(delta);
        let mut edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_id != from_id || edge.to_id != to_id)
            .collect::<Vec<Edge>>()
            .clone();
        edges.push(edge);
        Graph {
            vertexes: self.vertexes.clone(),
            edges: edges,
        }
    }

    pub fn total(&self) -> (i64, i64) {
        self.edges.iter().fold((0i64, 0i64), |sum, edge| {
            (sum.0 + edge.flow, sum.1 + edge.capacity)
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeDirection {
    Forward,
    Backward,
}

#[derive(Clone, Debug)]
pub struct VertexInfo {
    pub previous: u32,
    pub direction: EdgeDirection,
}

pub fn new_graph() -> Graph {
    Graph {
        vertexes: vec![
            Vertex {
                id: 0,
                name: "s".to_string(),
                vertex_type: VertexType::Source,
            },
            Vertex {
                id: 1,
                name: "1".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 2,
                name: "2".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 3,
                name: "3".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 4,
                name: "4".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 9999,
                name: "t".to_string(),
                vertex_type: VertexType::Shink,
            },
        ],

        edges: vec![
            Edge {
                from_id: 0,
                to_id: 1,
                flow: 0,
                capacity: 3,
            },
            Edge {
                from_id: 0,
                to_id: 2,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 1,
                to_id: 4,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 2,
                to_id: 4,
                flow: 0,
                capacity: 3,
            },
            Edge {
                from_id: 1,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 2,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 3,
                to_id: 9999,
                flow: 0,
                capacity: 3,
            },
            Edge {
                from_id: 4,
                to_id: 9999,
                flow: 0,
                capacity: 2,
            },
        ],
    }
}

pub fn new_graph2() -> Graph {
    Graph {
        vertexes: vec![
            Vertex {
                id: 0,
                name: "s".to_string(),
                vertex_type: VertexType::Source,
            },
            Vertex {
                id: 1,
                name: "1".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 2,
                name: "2".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 3,
                name: "3".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 4,
                name: "4".to_string(),
                vertex_type: VertexType::None,
            },
            Vertex {
                id: 9999,
                name: "t".to_string(),
                vertex_type: VertexType::Shink,
            },
        ],

        edges: vec![
            Edge {
                from_id: 0,
                to_id: 1,
                flow: 0,
                capacity: 3,
            },
            Edge {
                from_id: 0,
                to_id: 2,
                flow: 2,
                capacity: 2,
            },
            Edge {
                from_id: 1,
                to_id: 4,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 2,
                to_id: 4,
                flow: 2,
                capacity: 3,
            },
            Edge {
                from_id: 1,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 2,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge {
                from_id: 3,
                to_id: 9999,
                flow: 0,
                capacity: 3,
            },
            Edge {
                from_id: 4,
                to_id: 9999,
                flow: 2,
                capacity: 2,
            },
        ],
    }
}

pub fn show_graph(graph: &Graph) {
    // mermaid stateDiagram
    let (flow, capacity) = graph.total();
    println!("flow/capacity = {}/{}", flow, capacity);
    println!("```mermaid");
    println!("stateDiagram-v2");
    println!("  direction LR;");
    let vs = &graph.vertexes;
    for i in 0..vs.len() {
        println!("  state \"{}\" as v_{}", vs[i].name, vs[i].id);
    }
    let edges = &graph.edges;
    for i in 0..edges.len() {
        println!(
            "  v_{} --> v_{}: {}/{}",
            edges[i].from_id, edges[i].to_id, edges[i].flow, edges[i].capacity
        );
    }
    println!("```");
}
