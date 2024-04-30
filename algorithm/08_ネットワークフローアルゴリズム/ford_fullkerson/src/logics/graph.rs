#[derive(Clone, Debug)]
pub struct Vertex {
    id: u32,
    name: String,
}

#[derive(Clone, Debug)]
pub struct Edge {
    from_id: u32,
    to_id: u32,
    flow: u32,
    capacity: u32,
}

#[derive(Clone, Debug)]
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
}

pub fn new_graph() -> Graph {
    Graph {
        vertexes: vec![
            Vertex { id: 0, name: "s".to_string() },
            Vertex { id: 1, name: "1".to_string() },
            Vertex { id: 2, name: "2".to_string() },
            Vertex { id: 3, name: "3".to_string() },
            Vertex { id: 4, name: "4".to_string() },
            Vertex {
                id: 9999,
                name: "t".to_string(),
            },
        ],

        edges: vec![
            Edge{
                from_id: 0 ,
                to_id: 1,
                flow: 0,
                capacity: 3,
            },
            Edge{
                from_id: 0 ,
                to_id: 2,
                flow: 0,
                capacity: 2,
            },
            Edge{
                from_id: 1 ,
                to_id: 4,
                flow: 0,
                capacity: 2,
            },
            Edge{
                from_id: 2 ,
                to_id: 4,
                flow: 0,
                capacity: 3,
            },
            Edge{
                from_id: 1 ,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge{
                from_id: 2 ,
                to_id: 3,
                flow: 0,
                capacity: 2,
            },
            Edge{
                from_id: 3 ,
                to_id: 9999,
                flow: 0,
                capacity: 3,
            },
            Edge{
                from_id: 4 ,
                to_id: 9999,
                flow: 0,
                capacity: 2,
            },
        ],
    }
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
        println!(
            "  v_{} --> v_{}: {}/{}",
            edges[i].from_id, edges[i].to_id, edges[i].flow, edges[i].capacity
        );
    }
    println!("```");
}
