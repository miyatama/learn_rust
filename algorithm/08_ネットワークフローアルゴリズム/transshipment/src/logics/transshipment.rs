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
    // max(wk)
    warehouse_size: i64,
    transship_cost: i64,
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
}

pub fn get_productions() -> Vec<Production> {
    let sources = vec![
        (60, 20, "生産拠点A".to_string()),
        (20, 10, "生産拠点B".to_string()),
        (60, 5, "生産拠点C".to_string()),
    ];
    sources
        .into_iter()
        .enumerate()
        .map(|(index, source)| Production {
            id: index as u32,
            supply: source.0,
            name: source.2.clone(),
        })
        .collect::<Vec<Production>>()
}
pub fn get_distributions() -> Vec<Distribution> {
    let sources = vec![
        ("倉庫A-1".to_string(), 100, 20),
        ("倉庫A-2".to_string(), 110, 19),
        ("倉庫B-1".to_string(), 120, 18),
        ("倉庫B-2".to_string(), 130, 17),
        ("倉庫B-3".to_string(), 140, 15),
        ("倉庫B-4".to_string(), 100, 8),
    ];
    sources
        .into_iter()
        .enumerate()
        .map(|(index, source)| Distribution {
            id: index as u32,
            name: source.0,
            warehouse_size: source.1,
            transship_cost: source.2,
        })
        .collect::<Vec<Distribution>>()
}

pub fn get_consumptions() -> Vec<Consumption> {
    let sources = vec![
        (100, "小売店A".to_string()),
        (30, "小売店B".to_string()),
        (10, "小売店C".to_string()),
    ];
    sources
        .into_iter()
        .enumerate()
        .map(|(index, source)| Consumption {
            id: index as u32,
            name: source.1,
            demand: source.0,
        })
        .collect::<Vec<Consumption>>()
}
pub fn get_routes() -> Vec<Route> {
    let mut routes: Vec<Route> = Vec::new();
    // 生産 -> 流通A
    let sources = vec![
        (0, 0, 150),
        (0, 1, 140),
        (1, 0, 130),
        (1, 1, 120),
        (2, 0, 110),
        (2, 1, 100),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_type: BaseType::Production,
            from_id: source.0,
            to_type: BaseType::Distribution,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .for_each(|value| {
            routes.push(value.clone());
        });

    // 流通A -> 流通B
    let sources = vec![
        (0, 2, 130),
        (0, 3, 140),
        (0, 4, 150),
        (0, 5, 160),
        (1, 2, 170),
        (1, 3, 180),
        (1, 4, 190),
        (1, 5, 200),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_type: BaseType::Distribution,
            from_id: source.0,
            to_type: BaseType::Distribution,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .for_each(|value| {
            routes.push(value.clone());
        });
    // 流通B -> 小売
    let sources = vec![
        (2, 0, 19),
        (2, 1, 18),
        (2, 2, 17),
        (3, 0, 16),
        (3, 1, 15),
        (3, 2, 14),
        (4, 0, 13),
        (4, 1, 12),
        (4, 2, 11),
        (5, 0, 10),
        (5, 1, 9),
        (5, 2, 8),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_type: BaseType::Distribution,
            from_id: source.0,
            to_type: BaseType::Consumption,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .for_each(|value| {
            routes.push(value.clone());
        });
    routes
}

pub fn get_productions2() -> Vec<Production> {
    let sources = vec![
        (600, 10, "生産拠点A".to_string()),
        (800, 15, "生産拠点B".to_string()),
    ];

    sources
        .into_iter()
        .enumerate()
        .map(|(index, source)| Production {
            id: index as u32,
            supply: source.0,
            name: source.2.clone(),
        })
        .collect::<Vec<Production>>()
}

pub fn get_consumptions2() -> Vec<Consumption> {
    let sources = vec![(300, "小売店A".to_string()), (300, "小売店B".to_string())];
    sources
        .into_iter()
        .enumerate()
        .map(|(index, source)| Consumption {
            id: index as u32,
            name: source.1,
            demand: source.0,
        })
        .collect::<Vec<Consumption>>()
}

pub fn get_routes2() -> Vec<Route> {
    let sources = vec![(0, 0, 7), (0, 1, 6), (1, 0, 4), (1, 1, 6)];
    sources
        .into_iter()
        .map(|source| Route {
            from_type: BaseType::Production,
            from_id: source.0,
            to_type: BaseType::Consumption,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .collect::<Vec<Route>>()
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

    let get_vertex_id = |base_type: &BaseType, id: u32| -> u32 {
        return match base_type {
            BaseType::Production => production_id_base,
            BaseType::Distribution => distribution_id_base,
            BaseType::Consumption => consumption_id_base,
        } + id;
    };

    // production -> consumption
    routes
        .iter()
        .filter(|route| {
            route.from_type == BaseType::Production && route.to_type == BaseType::Consumption
        })
        .for_each(|route| {
            edges.push(Edge {
                from_id: get_vertex_id(&route.from_type, route.from_id),
                to_id: get_vertex_id(&route.to_type, route.to_id),
                flow: 0,
                capacity: i64::MAX,
                cost: route.cost_par_product,
            });
        });

    // production -> distribution
    routes
        .iter()
        .filter(|route| {
            route.from_type == BaseType::Production && route.to_type == BaseType::Distribution
        })
        .for_each(|route| {
            let production = productions
                .iter()
                .filter(|value| value.id == route.from_id)
                .next()
                .unwrap();
            edges.push(Edge {
                from_id: get_vertex_id(&route.from_type, route.from_id),
                to_id: get_vertex_id(&route.to_type, route.to_id),
                flow: 0,
                capacity: production.supply,
                cost: route.cost_par_product,
            });
        });

    // distribution -> distribution
    routes
        .iter()
        .filter(|route| {
            route.from_type == BaseType::Distribution && route.to_type == BaseType::Distribution
        })
        .for_each(|route| {
            let distribution = distributions
                .iter()
                .filter(|value| value.id == route.from_id)
                .next()
                .unwrap();
            edges.push(Edge {
                from_id: get_vertex_id(&route.from_type, route.from_id),
                to_id: get_vertex_id(&route.to_type, route.to_id),
                flow: 0,
                capacity: distribution.warehouse_size,
                cost: distribution.transship_cost,
            });
        });

    // distribution -> consumption
    routes
        .iter()
        .filter(|route| {
            route.from_type == BaseType::Distribution && route.to_type == BaseType::Consumption
        })
        .for_each(|route| {
            let distribution = distributions
                .iter()
                .filter(|value| value.id == route.from_id)
                .next()
                .unwrap();
            edges.push(Edge {
                from_id: get_vertex_id(&route.from_type, route.from_id),
                to_id: get_vertex_id(&route.to_type, route.to_id),
                flow: 0,
                capacity: distribution.warehouse_size,
                cost: route.cost_par_product,
            });
        });

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
