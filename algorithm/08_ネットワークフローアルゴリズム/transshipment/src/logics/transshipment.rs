use super::graph::{Edge, Graph, Vertex, VertexType};

// 各需要基地向けのコスト
#[derive(Clone, Debug)]
pub struct ToConsumptionCost {
    id: u32,
    cost: i64,
}

#[derive(Clone, Debug)]
pub struct Production {
    id: u32,
    supply: i64,
    name: String,
    to_consumption_costs: Vec<ToConsumptionCost>,
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
    from_id: u32,
    to_id: u32,
    cost_par_product: i64,
}

pub fn get_productions() -> Vec<Production> {
    let sources = vec![
        (
            1000,
            60,
            "生産拠点A".to_string(),
            vec![(3000, 100), (3001, 200), (3002, 300)],
        ),
        (
            1100,
            20,
            "生産拠点B".to_string(),
            vec![(3000, 100), (3001, 200), (3002, 300)],
        ),
        (
            1200,
            60,
            "生産拠点C".to_string(),
            vec![(3000, 100), (3001, 200), (3002, 300)],
        ),
    ];
    sources
        .into_iter()
        .map(|source| Production {
            id: source.0,
            supply: source.1,
            name: source.2.clone(),
            to_consumption_costs: source
                .3
                .iter()
                .map(|value| ToConsumptionCost {
                    id: value.0,
                    cost: value.1,
                })
                .collect::<Vec<ToConsumptionCost>>(),
        })
        .collect::<Vec<Production>>()
}
pub fn get_distributions() -> Vec<Distribution> {
    let sources = vec![
        (2000, "倉庫A-1".to_string(), 100, 20),
        (2100, "倉庫A-2".to_string(), 110, 19),
        (2500, "倉庫B-1".to_string(), 120, 18),
        (2510, "倉庫B-2".to_string(), 130, 17),
        (2520, "倉庫B-3".to_string(), 140, 15),
        (2530, "倉庫B-4".to_string(), 100, 8),
    ];
    sources
        .into_iter()
        .map(|source| Distribution {
            id: source.0,
            name: source.1,
            warehouse_size: source.2,
            transship_cost: source.3,
        })
        .collect::<Vec<Distribution>>()
}

pub fn get_consumptions() -> Vec<Consumption> {
    let sources = vec![
        (3000, 100, "小売店A".to_string()),
        (3001, 30, "小売店B".to_string()),
        (3002, 10, "小売店C".to_string()),
    ];
    sources
        .into_iter()
        .map(|source| Consumption {
            id: source.0,
            name: source.2,
            demand: source.1,
        })
        .collect::<Vec<Consumption>>()
}
pub fn get_routes() -> Vec<Route> {
    let mut routes: Vec<Route> = Vec::new();
    // 生産 -> 流通A
    let sources = vec![
        (1000, 2000, 150),
        (1000, 2100, 140),
        (1100, 2000, 130),
        (1100, 2100, 120),
        (1200, 2000, 110),
        (1200, 2100, 100),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_id: source.0,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .for_each(|value| {
            routes.push(value.clone());
        });

    // 流通A -> 流通B
    let sources = vec![
        (2000, 2500, 130),
        (2000, 2510, 140),
        (2000, 2520, 150),
        (2000, 2530, 160),
        (2100, 2500, 170),
        (2100, 2510, 180),
        (2100, 2520, 190),
        (2100, 2530, 200),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_id: source.0,
            to_id: source.1,
            cost_par_product: source.2,
        })
        .for_each(|value| {
            routes.push(value.clone());
        });
    // 流通B -> 小売
    let sources = vec![
        (2500, 3000, 19),
        (2500, 3001, 18),
        (2500, 3002, 17),
        (2510, 3000, 16),
        (2510, 3001, 15),
        (2510, 3002, 14),
        (2520, 3000, 13),
        (2520, 3001, 12),
        (2520, 3002, 11),
        (2530, 3000, 10),
        (2530, 3001, 9),
        (2530, 3002, 8),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_id: source.0,
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
        (
            1000,
            600,
            "生産拠点A".to_string(),
            vec![(3000, 20), (3100, 30)],
        ),
        (
            1100,
            800,
            "生産拠点B".to_string(),
            vec![(3000, 20), (3100, 30)],
        ),
    ];

    sources
        .into_iter()
        .map(|source| Production {
            id: source.0,
            supply: source.1,
            name: source.2.clone(),
            to_consumption_costs: source
                .3
                .iter()
                .map(|value| ToConsumptionCost {
                    id: value.0,
                    cost: value.1,
                })
                .collect::<Vec<ToConsumptionCost>>(),
        })
        .collect::<Vec<Production>>()
}

pub fn get_consumptions2() -> Vec<Consumption> {
    let sources = vec![
        (3000, 300, "小売店A".to_string()),
        (3100, 300, "小売店B".to_string()),
    ];
    sources
        .into_iter()
        .map(|source| Consumption {
            id: source.0,
            name: source.2,
            demand: source.1,
        })
        .collect::<Vec<Consumption>>()
}

pub fn get_routes2() -> Vec<Route> {
    let sources = vec![
        (1000, 3000, 7),
        (1000, 3100, 6),
        (1100, 3000, 4),
        (1100, 3100, 6),
    ];
    sources
        .into_iter()
        .map(|source| Route {
            from_id: source.0,
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

    // vertexesの構築
    // source
    let source_id = 0;
    vertexes.push(Vertex {
        id: source_id,
        name: "s".to_string(),
        vertex_type: VertexType::Source,
    });

    // productions: [1]～[1 + productions.len()]
    // index = i
    for i in 0..productions.len() {
        vertexes.push(Vertex {
            id: productions[i].id,
            name: productions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    // distributions
    // index = m + 2 * k - 1, index = m + 2 * k
    for i in 0..distributions.len() {
        vertexes.push(Vertex {
            id: distributions[i].id,
            name: distributions[i].name.clone(),
            vertex_type: VertexType::None,
        });
        vertexes.push(Vertex {
            id: distributions[i].id,
            name: distributions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    // consumptions
    // index = m + 2 * w + j
    for i in 0..consumptions.len() {
        vertexes.push(Vertex {
            id: consumptions[i].id,
            name: consumptions[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    // shink
    // index = n + m + 2 * w + 1
    let shink_id = 99999;
    let shink_index = productions.len() + consumptions.len() + 2 * distributions.len() + 1;
    vertexes.push(Vertex {
        id: shink_id,
        name: "t".to_string(),
        vertex_type: VertexType::Shink,
    });

    // 辺の構築
    let mut edges: Vec<Edge> = Vec::new();
    // source -> productions
    for i in 0..productions.len() {
        edges.push(Edge {
            from_vertex_index: 0,
            to_vertex_index: i + 1,
            flow: 0,
            capacity: productions[i].supply,
            cost: 0,
        });
    }

    // index = m + 2 * w + j
    let get_production = |id: u32, productions: &Vec<Production>| -> Production {
        productions
            .iter()
            .find(|value| value.id == id)
            .unwrap()
            .clone()
    };
    let get_production_vertex_index = |id: u32, productions: &Vec<Production>| -> usize {
        productions
            .iter()
            .enumerate()
            .find(|(_, value)| value.id == id)
            .unwrap()
            .0
            + 1
    };
    let get_consumption = |id: u32, consumptions: &Vec<Consumption>| -> Consumption {
        consumptions
            .iter()
            .find(|value| value.id == id)
            .unwrap()
            .clone()
    };
    let get_consumption_vertex_index = |id: u32,
                                        consumptions: &Vec<Consumption>,
                                        productions_size: usize,
                                        distributions_size: usize|
     -> usize {
        let index = consumptions
            .iter()
            .enumerate()
            .find(|(_, value)| value.id == id)
            .unwrap()
            .0;
        index + productions_size + distributions_size * 2
    };
    let get_distribution_vertex_indexes =
        |id: u32, distributions: &Vec<Distribution>, productions_size: usize| -> (usize, usize) {
            let index = distributions
                .iter()
                .enumerate()
                .find(|(_, value)| value.id == id)
                .unwrap()
                .0;
            (
                productions_size + 2 * (index + 1) - 1,
                productions_size + 2 * (index + 1),
            )
        };

    // productions -> consumptions
    for i in 0..productions.len() {
        let production = productions[i].clone();
        for j in 0..production.to_consumption_costs.len() {
            let to_consumption_cost = &production.to_consumption_costs[j];
            let vertex_index = get_consumption_vertex_index(
                to_consumption_cost.id,
                &consumptions,
                productions.len(),
                distributions.len(),
            );

            edges.push(Edge {
                from_vertex_index: i + 1,
                to_vertex_index: vertex_index,
                flow: 0,
                capacity: i64::MAX,
                cost: to_consumption_cost.cost,
            });
        }
    }

    // consumptions -> shink
    for j in 0..consumptions.len() {
        let consumption = consumptions[j].clone();
        edges.push(Edge {
            from_vertex_index: get_consumption_vertex_index(
                consumption.id,
                &consumptions,
                productions.len(),
                distributions.len(),
            ),
            to_vertex_index: shink_index,
            flow: 0,
            capacity: consumption.demand,
            cost: 0,
        });
    }

    // transshipment(same distribution)
    for k in 0..distributions.len() {
        let distribution = distributions[k].clone();
        let index1 = productions.len() + 2 * (k + 1) - 1;
        let index2 = productions.len() + 2 * (k + 1);
        edges.push(Edge {
            from_vertex_index: index1,
            to_vertex_index: index2,
            flow: 0,
            capacity: distribution.warehouse_size,
            cost: distribution.transship_cost,
        });
    }
    let is_production = |id: u32, productions: &Vec<Production>| -> bool {
        productions.iter().find(|value| value.id == id).is_some()
    };
    let is_consumption = |id: u32, consumptions: &Vec<Consumption>| -> bool {
        consumptions.iter().find(|value| value.id == id).is_some()
    };

    // 既存ルートの追加(倉庫を経由しないルートは存在しない)
    // production -> distribution -> consumption
    routes.iter().for_each(|route| {
        let from_production = is_production(route.from_id, &productions);
        let to_consumption = is_consumption(route.to_id, &consumptions);

        let (from_index, to_index, capacity) = if from_production {
            let (index0, _) =
                get_distribution_vertex_indexes(route.to_id, &distributions, productions.len());
            (
                get_production_vertex_index(route.from_id, &productions),
                index0,
                get_production(route.from_id, &productions).supply,
            )
        } else if to_consumption {
            let (_, index0) =
                get_distribution_vertex_indexes(route.from_id, &distributions, productions.len());
            (
                index0,
                get_consumption_vertex_index(
                    route.to_id,
                    &consumptions,
                    productions.len(),
                    distributions.len(),
                ),
                get_consumption(route.to_id, &consumptions).demand,
            )
        } else {
            let (_, index0) =
                get_distribution_vertex_indexes(route.from_id, &distributions, productions.len());
            let (index1, _) =
                get_distribution_vertex_indexes(route.to_id, &distributions, productions.len());
            (index0, index1, i64::MAX)
        };

        edges.push(Edge {
            from_vertex_index: from_index,
            to_vertex_index: to_index,
            flow: 0,
            capacity: capacity,
            cost: route.cost_par_product,
        });
    });

    Graph {
        vertexes: vertexes,
        edges: edges,
    }
}

pub fn show_result(graph: &Graph) {
    let total_capacity = graph.edges.iter().fold(0, |sum, edge| {
        if edge.capacity >= i64::MAX || sum >= i64::MAX {
            i64::MAX
        } else {
            sum + edge.capacity
        }
    });
    let max_flow = graph.edges.iter().fold(0, |sum, edge| sum + edge.flow);
    let min_cost = graph
        .edges
        .iter()
        .fold(0, |sum, edge| sum + edge.cost * edge.flow);
    println!("最大フロー: {} / {}", max_flow, total_capacity);
    println!("最小コスト: {}", min_cost);
}
