use super::graph::{Edge, Graph, Vertex, VertexType};

#[derive(Debug, PartialEq, Clone)]
pub enum BaseType {
    Production,
    Distribution,
    Consumption,
}

#[derive(Clone, Debug)]
pub struct Production {
    id: u32,
    supply: i64,
    name: String,
}

#[derive(Clone, Debug)]
pub struct Distribution {
    id: u32,
    name: String,
}

#[derive(Clone, Debug)]
pub struct Consumption {
    id: u32,
    name: String,
    demand: i64,
}

#[derive(Clone, Debug)]
pub struct Route {
    from_type: BaseType,
    from_id: u32,
    to_type: BaseType,
    to_id: u32,
    cost_par_product: i64,
    capacity: i64,
}

pub fn get_productions() -> Vec<Production> {
    vec![
        Production {
            id: 0,
            supply: 60,
            name: "生産拠点A".to_string(),
        },
        Production {
            id: 1,
            supply: 20,
            name: "生産拠点B".to_string(),
        },
        Production {
            id: 2,
            supply: 60,
            name: "生産拠点C".to_string(),
        },
    ]
}
pub fn get_distributions() -> Vec<Distribution> {
    vec![
        Distribution {
            id: 0,
            name: "倉庫A-1".to_string(),
        },
        Distribution {
            id: 1,
            name: "倉庫A-2".to_string(),
        },
        Distribution {
            id: 2,
            name: "倉庫B-1".to_string(),
        },
        Distribution {
            id: 3,
            name: "倉庫B-2".to_string(),
        },
        Distribution {
            id: 4,
            name: "倉庫B-3".to_string(),
        },
        Distribution {
            id: 5,
            name: "倉庫B-4".to_string(),
        },
    ]
}

pub fn get_consumptions() -> Vec<Consumption> {
    vec![
        Consumption {
            id: 0,
            name: "小売店A".to_string(),
            demand: 100,
        },
        Consumption {
            id: 1,
            name: "小売店B".to_string(),
            demand: 30,
        },
        Consumption {
            id: 2,
            name: "小売店C".to_string(),
            demand: 10,
        },
    ]
}
pub fn get_routes() -> Vec<Route> {
    vec![
        // 生産 -> 流通A
        Route {
            from_type: BaseType::Production,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 0,
            cost_par_product: 150,
            capacity: 1000,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 1,
            cost_par_product: 140,
            capacity: 1000,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 0,
            cost_par_product: 130,
            capacity: 1000,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 1,
            cost_par_product: 120,
            capacity: 1000,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 2,
            to_type: BaseType::Distribution,
            to_id: 0,
            cost_par_product: 110,
            capacity: 1000,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 2,
            to_type: BaseType::Distribution,
            to_id: 1,
            cost_par_product: 100,
            capacity: 1000,
        },
        // 流通A -> 流通B
        Route {
            from_type: BaseType::Distribution,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 2,
            cost_par_product: 130,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 3,
            cost_par_product: 140,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 4,
            cost_par_product: 150,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 0,
            to_type: BaseType::Distribution,
            to_id: 5,
            cost_par_product: 160,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 2,
            cost_par_product: 170,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 3,
            cost_par_product: 180,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 4,
            cost_par_product: 190,
            capacity: 500,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 1,
            to_type: BaseType::Distribution,
            to_id: 5,
            cost_par_product: 200,
            capacity: 500,
        },
        // 流通B -> 小売
        Route {
            from_type: BaseType::Distribution,
            from_id: 2,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 19,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 2,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 18,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 2,
            to_type: BaseType::Consumption,
            to_id: 2,
            cost_par_product: 17,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 3,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 16,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 3,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 15,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 3,
            to_type: BaseType::Consumption,
            to_id: 2,
            cost_par_product: 14,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 4,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 13,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 4,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 12,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 4,
            to_type: BaseType::Consumption,
            to_id: 2,
            cost_par_product: 11,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 5,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 10,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 5,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 9,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Distribution,
            from_id: 5,
            to_type: BaseType::Consumption,
            to_id: 2,
            cost_par_product: 8,
            capacity: 200,
        },
    ]
}


pub fn get_productions2() -> Vec<Production> {
    vec![
        Production {
            id: 0,
            supply: 600,
            name: "生産拠点A".to_string(),
        },
        Production {
            id: 1,
            supply: 800,
            name: "生産拠点B".to_string(),
        },
    ]
}

pub fn get_consumptions2() -> Vec<Consumption> {
    vec![
        Consumption {
            id: 0,
            name: "小売店A".to_string(),
            demand: 300,
        },
        Consumption {
            id: 1,
            name: "小売店B".to_string(),
            demand: 300,
        },
    ]
}
pub fn get_routes2() -> Vec<Route> {
    vec![
        Route {
            from_type: BaseType::Production,
            from_id: 0,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 7,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 0,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 6,
            capacity: 200,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 1,
            to_type: BaseType::Consumption,
            to_id: 0,
            cost_par_product: 4,
            capacity: 280,
        },
        Route {
            from_type: BaseType::Production,
            from_id: 1,
            to_type: BaseType::Consumption,
            to_id: 1,
            cost_par_product: 6,
            capacity: 350,
        },
    ]

}

pub fn to_graph(
    productions: &Vec<Production>,
    distributions: &Vec<Distribution>,
    consumptions: &Vec<Consumption>,
    routes: &Vec<Route>,
) -> Graph {
    let mut vertexes: Vec<Vertex> = Vec::new();

    // source, shinkを追加
    let source_id = 0;
    let shink_id = 99999;
    vertexes.push(Vertex {
        id: source_id,
        name: "s".to_string(),
        vertex_type: VertexType::Source,
    });
    vertexes.push(Vertex {
        id: shink_id,
        name: "t".to_string(),
        vertex_type: VertexType::Shink,
    });

    let production_id_base = 1000;
    for i in 0..productions.len() {
        vertexes.push(Vertex {
            id: production_id_base + productions[i].id,
            name: productions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    let distribution_id_base = 2000;
    for i in 0..distributions.len() {
        vertexes.push(Vertex {
            id: distribution_id_base + distributions[i].id,
            name: distributions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    let consumption_id_base = 3000;
    for i in 0..consumptions.len() {
        vertexes.push(Vertex {
            id: consumption_id_base + consumptions[i].id,
            name: consumptions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }

    let mut edges: Vec<Edge> = Vec::new();
    // source -> productions
    for i in 0..productions.len() {
        edges.push(Edge {
            from_id: 0,
            to_id: productions[i].id + production_id_base,
            flow: 0,
            capacity: productions[i].supply,
            cost: 0,
        });
    }

    // productions -> distributions -> consumptions
    for i in 0..routes.len() {
        let from_id = routes[i].from_id
            + match routes[i].from_type {
                BaseType::Production => production_id_base,
                BaseType::Distribution => distribution_id_base,
                BaseType::Consumption => consumption_id_base,
            };
        let to_id = routes[i].to_id
            + match routes[i].to_type {
                BaseType::Production => production_id_base,
                BaseType::Distribution => distribution_id_base,
                BaseType::Consumption => consumption_id_base,
            };
        edges.push(Edge {
            from_id: from_id,
            to_id: to_id,
            flow: 0,
            capacity: routes[i].capacity,
            cost: routes[i].cost_par_product,
        });
    }

    // consumptions -> shink
    for i in 0..consumptions.len() {
        edges.push(Edge {
            from_id: consumptions[i].id + consumption_id_base,
            to_id: shink_id,
            flow: 0,
            capacity: consumptions[i].demand,
            cost: 0,
        });
    }
    Graph {
        vertexes: vertexes,
        edges: edges,
    }
}

pub fn show_result(graph: &Graph) {
    let total_capacity = graph.edges.iter().fold(0, |sum, edge| sum + edge.capacity);
    let max_flow = graph.edges.iter().fold(0, |sum, edge| sum + edge.flow);
    let min_cost = graph
        .edges
        .iter()
        .fold(0, |sum, edge| sum + edge.cost * edge.flow);
    println!("最大フロー: {} / {}", max_flow, total_capacity);
    println!("最小コスト: {}", min_cost);
}
