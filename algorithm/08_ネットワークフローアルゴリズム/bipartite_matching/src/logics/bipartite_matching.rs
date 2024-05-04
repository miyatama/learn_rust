use super::graph::{Edge, Graph, Vertex, VertexType};

#[derive(Debug, Clone, PartialEq)]
pub enum License {
    BookKeeping,
    HighSchool,
    Ritch,
}

#[derive(Debug, Clone)]
pub struct Work {
    id: u32,
    name: String,
    need_licenses: Vec<License>,
}

#[derive(Debug, Clone)]
pub struct Human {
    id: u32,
    name: String,
    has_licenses: Vec<License>,
}

impl Human {
    pub fn requirement(&self, licenses: &Vec<License>) -> bool {
        for i in 0..licenses.len() {
            if self
                .has_licenses
                .iter()
                .filter(|&licence| *licence == licenses[i])
                .next()
                .is_none()
            {
                return false;
            }
        }
        true
    }
}

pub fn get_data() -> (Vec<Work>, Vec<Human>) {
    (get_works(), get_humans())
}

pub fn show_works(works: &Vec<Work>) {
    println!("業務");
    println!("| 名前 | 必要な資格 |");
    println!("| :----- | :----- |");
    for i in 0..works.len() {
        println!(
            "| {} | {} |",
            works[i].name,
            get_licenses_text(&works[i].need_licenses)
        );
    }
    println!("");
}

pub fn show_humans(works: &Vec<Human>) {
    println!("応募者");
    println!("| 名前 | 資格 |");
    println!("| :----- | :----- |");
    for i in 0..works.len() {
        println!(
            "| {} | {} |",
            works[i].name,
            get_licenses_text(&works[i].has_licenses)
        );
    }
    println!("");
}

pub fn to_graph(works: &Vec<Work>, humans: &Vec<Human>) -> Graph {
    let mut vertexes: Vec<Vertex> = Vec::new();
    // Shink, Sourceの追加
    let source_id = 0;
    vertexes.push(Vertex {
        id: source_id,
        name: "Source".to_string(),
        vertex_type: VertexType::Source,
    });
    let shink_id = 99999;
    vertexes.push(Vertex {
        id: shink_id,
        name: "Shink".to_string(),
        vertex_type: VertexType::Shink,
    });
    for i in 0..works.len() {
        vertexes.push(Vertex {
            id: works[i].id,
            name: works[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }
    for i in 0..humans.len() {
        vertexes.push(Vertex {
            id: humans[i].id,
            name: humans[i].name.clone(),
            vertex_type: VertexType::None,
        });
    }

    let mut edges: Vec<Edge> = Vec::new();
    // source -> work edges
    for i in 0..works.len() {
        edges.push(Edge {
            from_id: source_id,
            to_id: works[i].id,
            flow: 0,
            capacity: 1,
        });
    }

    // humans -> shink edges
    for i in 0..humans.len() {
        edges.push(Edge {
            from_id: humans[i].id,
            to_id: shink_id,
            flow: 0,
            capacity: 1,
        });
    }

    // work -> human edges
    for i in 0..works.len() {
        let work = &works[i];
        for j in 0..humans.len() {
            if humans[j].requirement(&work.need_licenses) {
                edges.push(Edge {
                    from_id: work.id,
                    to_id: humans[j].id,
                    flow: 0,
                    capacity: 1,
                })
            }
        }
    }

    Graph {
        vertexes: vertexes,
        edges: edges,
    }
}

pub fn show_assign(works: &Vec<Work>, humans: &Vec<Human>, graph: &Graph) {
    println!("人材割り当て");
    println!("| 業務 | 人材 | 必要資格 |");
    println!("| :----- | :----- | :----- |");
    for i in 0..works.len() {
        let work = &works[i];
        let human_ids = graph.get_active_vertex_id(work.id);
        let assigns = humans
            .iter()
            .filter(|human| {
                human_ids
                    .iter()
                    .filter(|&id| *id == human.id)
                    .next()
                    .is_some()
            })
            .map(|human| human.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        let licenses = get_licenses_text(&work.need_licenses);
        println!("| {} | {} | {} |", work.name, assigns, licenses);
    }
    println!("");
}

fn get_licenses_text(licenses: &Vec<License>) -> String {
    licenses
        .iter()
        .map(|license| match license {
            License::BookKeeping => "簿記".to_string(),
            License::HighSchool => "高校卒業資格".to_string(),
            License::Ritch => "金持ち".to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_works() -> Vec<Work> {
    vec![
        Work {
            id: 1000,
            name: "事務".to_string(),
            need_licenses: vec![License::HighSchool, License::BookKeeping],
        },
        Work {
            id: 1001,
            name: "エンジニア".to_string(),
            need_licenses: vec![License::HighSchool],
        },
        Work {
            id: 1002,
            name: "社長".to_string(),
            need_licenses: vec![License::Ritch],
        },
    ]
}

fn get_humans() -> Vec<Human> {
    vec![
        Human {
            id: 3000,
            name: "富田".to_string(),
            has_licenses: vec![License::BookKeeping, License::HighSchool, License::Ritch],
        },
        Human {
            id: 3001,
            name: "安田".to_string(),
            has_licenses: vec![License::HighSchool],
        },
        Human {
            id: 3002,
            name: "吉田".to_string(),
            has_licenses: vec![License::Ritch],
        },
        Human {
            id: 3003,
            name: "森田".to_string(),
            has_licenses: vec![License::HighSchool, License::Ritch],
        },
    ]
}
