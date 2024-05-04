use self::logics::{bipartite_matching, edmons_karp, ford_fullkerson, graph};

pub mod logics;

fn main() {
    let (works, humans) = bipartite_matching::get_data();
    bipartite_matching::show_works(&works);
    bipartite_matching::show_humans(&humans);

    let graph = bipartite_matching::to_graph(&works, &humans);
    graph::show_graph(&graph);

    println!("");
    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::maximum_flow(&graph.clone());
    graph::show_graph(&graph);
    bipartite_matching::show_assign(&works, &humans, &graph);
    println!("");

    println!("エドモンズ・カープ法");
    let graph = edmons_karp::maximum_flow(&graph.clone());
    graph::show_graph(&graph);
    bipartite_matching::show_assign(&works, &humans, &graph);
}
