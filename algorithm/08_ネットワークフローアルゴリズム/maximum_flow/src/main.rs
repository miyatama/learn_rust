use self::logics::{edmons_karp, ford_fullkerson, graph};

pub mod logics;

fn main() {
    let graph = graph::new_graph();
    graph::show_graph(&graph);

    // フォードファルカーソン法
    let graph = ford_fullkerson::maximum_flow(&graph.clone());
    graph::show_graph(&graph);

    // エドモンズ・カープ法
    let graph = edmons_karp::maximum_flow(&graph.clone());
    graph::show_graph(&graph);

    let graph = graph::new_graph2();
    graph::show_graph(&graph);

    // フォードファルカーソン法
    let graph = ford_fullkerson::maximum_flow(&graph.clone());
    graph::show_graph(&graph);

    // エドモンズ・カープ法
    let graph = edmons_karp::maximum_flow(&graph.clone());
    graph::show_graph(&graph);
}
