use self::logics::{ford_fullkerson, graph, minimal_cost_flow, shortest_path};

pub mod logics;

fn main() {
    let productions = minimal_cost_flow::get_productions();
    let distributions = minimal_cost_flow::get_distributions();
    let consumption = minimal_cost_flow::get_consumptions();
    let routes = minimal_cost_flow::get_routes();
    let graph = minimal_cost_flow::to_graph(&productions, &distributions, &consumption, &routes);
    graph::show_graph(&graph);

    println!("");
    println!("最短パス");
    let graph = shortest_path::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    println!("エドモンズカープ法");
    let graph = ford_fullkerson::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    let productions = minimal_cost_flow::get_productions2();
    let consumption = minimal_cost_flow::get_consumptions2();
    let routes = minimal_cost_flow::get_routes2();
    let graph = minimal_cost_flow::to_graph(&productions, &vec![], &consumption, &routes);
    graph::show_graph(&graph);

    println!("");
    println!("最短パス");
    let graph = shortest_path::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    println!("フォードファルカーソン法");
    let graph = ford_fullkerson::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");

    println!("エドモンズカープ法");
    let graph = ford_fullkerson::minimal_cost_flow(&graph.clone());
    minimal_cost_flow::show_result(&graph);
    graph::show_graph(&graph);
    println!("");
}
