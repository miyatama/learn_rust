use self::logics::{graph, ford_fullkerson};

pub mod logics;

fn main() {
    let graph = graph::new_graph();
    graph::show_graph(&graph);
    let graph = ford_fullkerson::maximum_flow(&graph);
    graph::show_graph(&graph);
}
