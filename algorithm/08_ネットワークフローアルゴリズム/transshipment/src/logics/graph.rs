use std::collections::HashSet;

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
    pub from_vertex_index: usize,
    pub to_vertex_index: usize,
    pub flow: i64,
    pub cost: i64,
    pub capacity: i64,
}

impl Edge {
    pub fn add_flow(&self, delta: i64) -> Edge {
        Edge {
            from_vertex_index: self.from_vertex_index,
            to_vertex_index: self.to_vertex_index,
            flow: self.flow + delta,
            capacity: self.capacity,
            cost: self.cost,
        }
    }

    pub fn get_cost(&self) -> i64 {
        self.cost * self.flow
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn get_vertex(&self, index: usize) -> Vertex {
        self.vertexes[index].clone()
    }

    pub fn get_edge(&self, from_vertex_index: usize, to_vertex_index: usize) -> Edge {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| {
                edge.from_vertex_index == from_vertex_index
                    && edge.to_vertex_index == to_vertex_index
            })
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_active_vertex_id(&self, from_vertex_index: usize) -> Vec<usize> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_vertex_index == from_vertex_index && edge.flow > 0)
            .map(|edge| edge.to_vertex_index)
            .collect::<Vec<usize>>()
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

    pub fn get_forward_edges(&self, vertex_index: usize) -> Vec<Edge> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.from_vertex_index == vertex_index)
            .collect::<Vec<Edge>>()
            .clone()
    }

    pub fn get_backward_edges(&self, vertex_index: usize) -> Vec<Edge> {
        self.edges
            .clone()
            .into_iter()
            .filter(|edge| edge.to_vertex_index == vertex_index)
            .collect::<Vec<Edge>>()
            .clone()
    }

    pub fn get_forward_edge(
        &self,
        from_vertex_index: usize,
        to_vertex_index: usize,
    ) -> Option<Edge> {
        self.edges.clone().into_iter().find(|edge| {
            edge.from_vertex_index == from_vertex_index && edge.to_vertex_index == to_vertex_index
        })
    }

    pub fn get_backward_edge(
        &self,
        from_vertex_index: usize,
        to_vertex_index: usize,
    ) -> Option<Edge> {
        self.edges.clone().into_iter().find(|edge| {
            edge.from_vertex_index == from_vertex_index && edge.to_vertex_index == to_vertex_index
        })
    }

    pub fn apply_flow(
        &self,
        from_vertex_index: usize,
        to_vertex_index: usize,
        delta: i64,
    ) -> Graph {
        let edge = self
            .get_edge(from_vertex_index, to_vertex_index)
            .add_flow(delta);
        let mut edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|edge| {
                edge.from_vertex_index != from_vertex_index
                    || edge.to_vertex_index != to_vertex_index
            })
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
    pub previous: usize,
    pub direction: EdgeDirection,
}

pub fn show_graph(graph: &Graph, show_zero_flow: bool) {
    // mermaid stateDiagram
    println!("```mermaid");
    println!("stateDiagram-v2");
    println!("  direction LR;");
    let mut hs: HashSet<u32> = HashSet::new();
    let vs = &graph.vertexes;
    for i in 0..vs.len() {
        if !hs.contains(&vs[i].id) {
            println!("  state \"{}\" as v_{}", vs[i].name, vs[i].id);
            hs.insert(vs[i].id);
        }
    }
    let edges = &graph.edges;
    for i in 0..edges.len() {
        if show_zero_flow || edges[i].flow > 0 {
            println!(
                "  v_{} --> v_{}: {}/{}",
                &graph.vertexes[edges[i].from_vertex_index].id,
                &graph.vertexes[edges[i].to_vertex_index].id,
                edges[i].flow,
                if edges[i].capacity >= i64::MAX {
                    "∞".to_string()
                } else {
                    edges[i].capacity.to_string()
                },
            );
        } else {
            println!(
                "  v_{} --> v_{}",
                &graph.vertexes[edges[i].from_vertex_index].id,
                &graph.vertexes[edges[i].to_vertex_index].id,
            );
        }
    }
    println!("```");
    println!("");
}
