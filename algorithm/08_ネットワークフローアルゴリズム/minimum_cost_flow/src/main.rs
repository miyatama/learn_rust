use self::logics::{edmons_karp, ford_fullkerson, graph, minimal_cost_flow};

pub mod logics;

fn main() {
    let productions = minimal_cost_flow::get_productions();
    let distributions = minimal_cost_flow::get_distributions();
    let consumption = minimal_cost_flow::get_consumptions();
    let routes = minimal_cost_flow::get_routes();
    let graph = minimal_cost_flow::to_graph(&productions, &distributions, &consumption, &routes);
    graph::show_graph(&graph);

    println!("");
    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    println!("エドモンズ・カープ法");
    let graph = edmons_karp::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
}
