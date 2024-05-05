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
    pub cost: i64,
    pub capacity: i64,
}

impl Edge {
    pub fn add_flow(&self, delta: i64) -> Edge {
        Edge {
            from_id: self.from_id,
            to_id: self.to_id,
            flow: self.flow + delta,
            capacity: self.capacity,
            cost: self.cost,
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

    pub fn get_active_vertex_id(&self, from_id: u32) -> Vec<u32> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_id == from_id && edge.flow > 0)
            .map(|edge| edge.to_id)
            .collect::<Vec<u32>>()
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

pub fn show_graph(graph: &Graph) {
    // mermaid stateDiagram
    println!("```mermaid");
    println!("stateDiagram-v2");
    println!("  direction LR;");
    let vs = &graph.vertexes;
    for i in 0..vs.len() {
        println!("  state \"{}\" as v_{}", vs[i].name, vs[i].id);
    }
    let edges = &graph.edges;
    for i in 0..edges.len() {
        if edges[i].flow > 0 {
            println!(
                "  v_{} --> v_{}: {}/{}",
                edges[i].from_id, edges[i].to_id, edges[i].flow, edges[i].capacity
            );
        } else {
            println!("  v_{} --> v_{}", edges[i].from_id, edges[i].to_id);
        }
    }
    println!("```");
    println!("");
}
