use std::collections::VecDeque;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
enum VertexStatus {
    // 未調査
    NotSearch,
    // 調査中
    Searching,
    // 調査済み
    Searched,
}

#[derive(Debug, Clone)]
struct Vertex {
    id: u64,
    edges: Vec<u64>,
    status: VertexStatus,
    from: Option<u64>,
}

pub fn search<W: Write>(w: &mut W, _n: u64, s: u64, t: u64, vertexes: Vec<(u64, Vec<u64>)>) {
    let vertexes = vertexes.iter().map(|val| parse_to_vertex(&val)).collect();
    let (route, vertexes) = search_route(s, t, &vertexes);
    print_vertexes_status(&vertexes);
    match route {
        None => writeln!(w, "route not found").unwrap(),
        Some(route) => {
            eprintln!("route: {:?}", generate_route_text(&route));
            route.iter().for_each(|val| {
                writeln!(w, "{}", val.id).unwrap();
            });
        }
    }
}

fn search_route(s: u64, t: u64, vertexes: &Vec<Vertex>) -> (Option<Vec<Vertex>>, Vec<Vertex>) {
    let mut queue = VecDeque::<u64>::new();
    queue.push_back(s);
    let mut vertexes = update_vertex_status(s, VertexStatus::Searching, None, vertexes);
    while !queue.is_empty() {
        let id = queue.pop_front().unwrap();

        let vertex = vertexes.iter().filter(|val| val.id == id).next().unwrap().clone();

        let child_ids = vertex
            .edges
            .iter()
            .filter(|id| {
                vertexes
                    .iter()
                    .filter(|vertex| vertex.id == **id && vertex.status == VertexStatus::NotSearch)
                    .next()
                    .is_some()
            })
            .map(|val| *val)
            .collect::<Vec<_>>();
        for i in 0..child_ids.len() {
            vertexes =
                update_vertex_status(child_ids[i], VertexStatus::Searching, Some(id), &vertexes);
            queue.push_back(child_ids[i]);
        }
        vertexes = update_vertex_status(id, VertexStatus::Searched, vertex.from, &vertexes);
        if id == t {
            break;
        }
    }

    let mut route = Vec::<Vertex>::new();
    let mut search_id = t;
    loop {
        let vertex = vertexes
            .iter()
            .filter(|val| val.id == search_id)
            .next()
            .unwrap();
        route.push(vertex.clone());
        if vertex.from.is_none() {
            break;
        }
        search_id = vertex.from.unwrap();
    }
    let route = route
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .to_vec();
    if route[0].id == s {
        (Some(route), vertexes)
    } else {
        (None, vertexes)
    }
}

fn generate_route_text(route: &Vec<Vertex>) -> String {
    route
        .iter()
        .map(|vertex| vertex.id.to_string())
        .collect::<Vec<String>>()
        .join(" -> ")
}

fn print_vertexes_status(vertexes: &Vec<Vertex>) {
    vertexes.iter().for_each(|vertex| {
        eprintln!("id: {} - {:?}", vertex.id, vertex.status);
    });
}

fn update_vertex_status(
    id: u64,
    status: VertexStatus,
    from: Option<u64>,
    vertexes: &Vec<Vertex>,
) -> Vec<Vertex> {
    vertexes
        .to_vec()
        .iter()
        .map(|val| {
            if val.id == id {
                Vertex {
                    id: val.id,
                    edges: val.edges.clone(),
                    status: status,
                    from: from,
                }
            } else {
                val.clone()
            }
        })
        .collect()
}

fn parse_to_vertex(src: &(u64, Vec<u64>)) -> Vertex {
    Vertex {
        id: src.0,
        edges: src.1.clone(),
        status: VertexStatus::NotSearch,
        from: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_01() {
        let mut buff = Vec::<u8>::new();
        search(&mut buff, 2, 1, 2, vec![(1, vec![2]), (2, vec![])]);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1", "2"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_search_02() {
        let mut buff = Vec::<u8>::new();
        search(
            &mut buff,
            16,
            0,
            15,
            vec![
                (0, vec![1, 6, 8]),
                (1, vec![0, 2, 3]),
                (6, vec![0, 5, 7]),
                (8, vec![0, 7, 14]),
                (2, vec![1, 11, 10]),
                (3, vec![1, 4, 12]),
                (5, vec![4, 6, 9]),
                (4, vec![3, 5, 13]),
                (7, vec![6, 8, 9]),
                (9, vec![5, 7, 15]),
                (10, vec![2]),
                (11, vec![2]),
                (12, vec![3]),
                (13, vec![4]),
                (14, vec![8]),
                (15, vec![9]),
            ],
        );
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0", "6", "5", "9", "15"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
