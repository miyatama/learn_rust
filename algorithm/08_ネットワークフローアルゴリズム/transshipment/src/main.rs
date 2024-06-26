use self::logics::{ford_fullkerson, graph, transshipment};

pub mod logics;

fn main() {
    /*
    let productions = transshipment::get_productions();
    let distributions = transshipment::get_distributions();
    let consumption = transshipment::get_consumptions();
    let routes = transshipment::get_routes();
    transshipment::show_transshipment(&productions, &distributions, &consumption, &routes);
    let graph = transshipment::to_graph(&productions, &distributions, &consumption, &routes);
    graph::show_graph(&graph, true);
    println!("");

    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::transshipment(&graph.clone());
    transshipment::show_result(&graph);
    graph::show_graph(&graph, false);
    println!("");

    println!("エドモンズカープ法");
    let graph = ford_fullkerson::transshipment(&graph.clone());
    transshipment::show_result(&graph);
    graph::show_graph(&graph, false);
    println!("");
     */

    let productions = transshipment::get_productions2();
    let distributions = transshipment::get_distributions2();
    let consumption = transshipment::get_consumptions2();
    let routes = transshipment::get_routes2();
    let graph = transshipment::to_graph(&productions, &distributions, &consumption, &routes);
    transshipment::show_transshipment(&productions, &distributions, &consumption, &routes);
    println!("");

    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::transshipment(&graph.clone());
    transshipment::show_result(&graph);
    graph::show_graph(&graph, false);
    println!("");

    println!("エドモンズカープ法");
    let graph = ford_fullkerson::transshipment(&graph.clone());
    transshipment::show_result(&graph);
    graph::show_graph(&graph, false);
    println!("");
}
