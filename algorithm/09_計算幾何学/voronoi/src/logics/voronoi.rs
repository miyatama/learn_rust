use super::common::{Line, LinePointDirection, Point, Polygon};
use rand::{thread_rng, Rng};
use std::collections::{HashSet, VecDeque};
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Line as SvgLine;
use svg::node::element::Rectangle as SvgRectangle;
use svg::node::element::Text as SvgText;
use svg::Document;

#[derive(Debug)]
struct Event {
    site: Option<Point>,
    event_type: EventType,
}

#[derive(Debug)]
enum EventType {
    Site,
    Circle,
}

/**
 * 点の集合からボロノイ辺を作成する
 */
pub fn calc_voronoi_lines(width: f64, height: f64, points: &Vec<Point>) -> Vec<Polygon> {
    // 点の数が2以下の場合は固定で線分を引く
    if points.len() <= 1 {
        return create_point_one_voronoi(width, height);
    } else if points.len() <= 2 {
        return create_point_twe_voronoi(width, height, &points);
    }

    // 作成されるボロノイ
    let mut voronoi: Vec<Polygon> = points
        .iter()
        .map(|point| Polygon::new(point.id))
        .collect::<Vec<Polygon>>();

    let points = add_dummy_points(width, height, &points);

    // 汀線の状態を保持する。ポイントのインデックスを汀線順に並べる。
    // intersections: 汀線に対応するポイントのインデックス
    let mut intersections: Vec<usize> = Vec::new();
    intersections.push(0);
    intersections.push(1);
    intersections.push(1);
    intersections.push(0);

    let last_event_timing = 2.0 * height;
    let site_event_timing = get_site_event_timing(last_event_timing, &points);
    let mut previous_event_timing = site_event_timing[1];
    let mut present_event_timing = site_event_timing[2];
    let mut event_type: EventType = EventType::Site;
    let mut next_generating_point_index = 2;

    let mut next_circle_event_timing = last_event_timing;
    let mut exception_index: Vec<usize> = Vec::new();

    while present_event_timing < last_event_timing {
        eprintln!("event: {:?}, timing: {}", event_type, present_event_timing);
        let mut intersection_num = intersections.len() / 2;

        // 隣り合う交点の位置を算出
        // intersection_pos: 汀線の交点座標(サイズ = intersections.size / 2.0)
        let mut intersection_pos: Vec<Point> = vec![Point::new(); intersection_num];
        for i in 0..intersection_num {
            let index1 = intersections[2 * i];
            let index2 = intersections[2 * i + 1];
            let start = get_intersection(&points, index1, index2, previous_event_timing);
            let end = get_intersection(&points, index1, index2, present_event_timing);

            let line = Line::new(start.x, start.y, end.x, end.y);
            if start.x != end.x || start.y != end.y {
                voronoi = add_line_to_polygons(&voronoi, &points, &line, index1, index2);
            }
            intersection_pos[i] = end.clone(); // 交点位置の更新のため、交点位置を保持しておく
        }

        // intersectionsの更新
        match event_type {
            EventType::Site => {
                let mut medial_insert_flag = 0;
                let mut insert_position = intersection_num;
                for i in 0..intersection_num {
                    if (points[next_generating_point_index].x - intersection_pos[i].x).abs() < 0.001
                    {
                        insert_position = i;
                        medial_insert_flag = 1;
                        exception_index.push(next_generating_point_index);
                        break;
                    }
                    if points[next_generating_point_index].x < intersection_pos[i].x {
                        insert_position = i;
                        break;
                    }
                }
                // 新たな交点の位置を挿入
                if medial_insert_flag == 1 {
                    // 新しく追加される母点のx座標がある交点の位置のx座標が一致する場合
                    intersections.insert(2 * insert_position + 1, next_generating_point_index);
                    intersections.insert(2 * (insert_position + 1), next_generating_point_index);
                } else if insert_position == 0 {
                    // 挿入位置が一番左側の場合
                    let outer_index = intersections[0];
                    intersections.insert(0, outer_index);
                    intersections.insert(1, next_generating_point_index);
                    intersections.insert(2, next_generating_point_index);
                    intersections.insert(3, outer_index);
                } else if insert_position == intersection_num {
                    // 挿入位置が一番右側の場合
                    let inner_index = intersections[2 * (insert_position - 1) + 1];
                    intersections.push(inner_index);
                    intersections.push(next_generating_point_index);
                    intersections.push(next_generating_point_index);
                    intersections.push(inner_index);
                } else {
                    // 挿入位置が配列の位置の間
                    let inner_index = intersections[2 * (insert_position - 1) + 1];
                    let outer_index = intersections[2 * insert_position];
                    intersections.insert(2 * insert_position, inner_index);
                    intersections.insert(2 * insert_position + 1, next_generating_point_index);
                    intersections.insert(2 * (insert_position + 1), next_generating_point_index);
                    intersections.insert(2 * (insert_position + 1) + 1, outer_index);
                }
                // 次の母点にindexを更新
                next_generating_point_index += 1;
                // 汀線上の交点の数を更新
                intersection_num += 1;
            }
            EventType::Circle => {
                // 隣り合う交点の位置が重なった場合、交点同士を合体させる
                let mut remove_index: Vec<usize> = Vec::new();
                for i in 1..intersection_num {
                    if !exception_index
                        .iter()
                        .any(|&index| index == intersections[2 * i])
                        && intersections[2 * i + 1] != intersections[2 * (i - 1)]
                        && intersection_pos[i].dist(&intersection_pos[i - 1]) < 2.0
                    {
                        remove_index.push(i);
                    }
                }
                for i in (0..remove_index.len()).rev() {
                    intersections.remove(2 * remove_index[i]);
                    intersections.remove(2 * (remove_index[i] - 1) + 1);
                    intersection_num -= 1;
                }
            }
        }

        next_circle_event_timing = last_event_timing;
        for j in 1..intersection_num {
            // 3つの母点のindexを取得
            let mut circle_point_index = vec![0usize; 3];
            circle_point_index[0] = intersections[2 * (j - 1)];
            circle_point_index[1] = intersections[2 * j];
            circle_point_index[2] = intersections[2 * j + 1];
            // 選択された隣り合った2つの交点のindexが異なる場合計算する
            if circle_point_index[0] != circle_point_index[2] {
                // 選択された3点を通る円の中心と半径を算出する
                let mut circle_radius = 0.0;
                // 関数を挿入
                let mut circle_point: Vec<Point> = vec![Point::new(); 3];
                for k in 0..3 {
                    circle_point[k] = points[circle_point_index[k]].clone();
                }
                let circle_center = get_circle_center(&circle_point);
                circle_radius = circle_point[0].dist(&circle_center.clone());
                // 「円の中心のy座標＋半径」の値が現状のイベントタイミング以上かつ次のサークルイベント発生タイミングより小さい値であれば、
                // 次のサークルイベント発生タイミングを「円の中心のy座標＋半径」の値に更新する
                match event_type {
                    EventType::Site => {
                        if !exception_index
                            .iter()
                            .any(|&index| index == circle_point_index[1])
                            && (circle_center.y + circle_radius) > present_event_timing - 0.001
                            && circle_center.y + circle_radius <= next_circle_event_timing
                        {
                            next_circle_event_timing = circle_center.y + circle_radius;
                        }
                    }
                    EventType::Circle => {
                        if present_event_timing < circle_center.y + circle_radius
                            && circle_center.y + circle_radius < next_circle_event_timing
                        {
                            next_circle_event_timing = circle_center.y + circle_radius;
                        }
                    }
                }
            }
        }

        previous_event_timing = present_event_timing;
        if next_circle_event_timing >= site_event_timing[next_generating_point_index] {
            present_event_timing = site_event_timing[next_generating_point_index];
            event_type = EventType::Site;
        } else {
            present_event_timing = next_circle_event_timing;
            event_type = EventType::Circle;
        }
        if (present_event_timing - previous_event_timing).abs() > 0.001 {
            exception_index.clear();
        }
    }

    // TODO ここを飛ばして13焦点でのヘンな線がなくなるか確認
    // 最後に残った境界線を描く
    eprintln!("event: last, timing: {}", present_event_timing);
    for j in 0..(intersections.len() / 2) {
        let index1 = intersections[2 * j];
        let index2 = intersections[2 * j + 1];
        let start = get_intersection(&points, index1, index2, previous_event_timing);
        let end = get_intersection(&points, index1, index2, present_event_timing);

        let line = Line::new(start.x, start.y, end.x, end.y);
        voronoi = add_line_to_polygons(&voronoi, &points, &line, index1, index2);
    }
    // print_polygons(&voronoi);
    let voronoi = shape_polygons(width, height, &voronoi);
    // print_polygons(&voronoi);
    voronoi
}

fn add_line_to_polygons(
    polygons: &Vec<Polygon>,
    points: &Vec<Point>,
    line: &Line,
    index1: usize,
    index2: usize,
) -> Vec<Polygon> {
    let mut polygons = polygons.clone();
    let mut add_point_ids: Vec<u32> = Vec::new();
    for index in vec![index1, index2] {
        let id = points[index].id;
        if id == 0 {
            continue;
        }
        add_point_ids.push(id);
        let voronoi_index = polygons
            .iter()
            .position(|voronoi| voronoi.point_id == id)
            .unwrap();
        let mut polygon_line = polygons[voronoi_index].lines.clone();
        polygon_line.push(line.clone());
        polygons[voronoi_index] = Polygon {
            point_id: id,
            lines: polygon_line,
        };
    }

    eprintln!(
        "point {:?} add line ({}, {}) to ({}, {})",
        add_point_ids, line.p1.x, line.p1.y, line.p2.x, line.p2.y
    );
    polygons
}

fn shape_polygons(width: f64, height: f64, polygons: &Vec<Polygon>) -> Vec<Polygon> {
    let mut polygons = polygons.clone();
    for i in 0..polygons.len() {
        polygons[i] = shape_polygon(width, height, &polygons[i].clone());
    }
    polygons
}

/**
 * やること
 * 1: 各線分の位置を下2桁まで有効とする
 * 2: 範囲を超える直線は範囲内に収める
 */
fn shape_polygon(width: f64, height: f64, polygon: &Polygon) -> Polygon {
    let trunc_line = |line: &Line| -> Line {
        Line {
            p1: Point {
                id: line.p1.id,
                x: trunc_f64(line.p1.x),
                y: trunc_f64(line.p1.y),
            },
            p2: Point {
                id: line.p2.id,
                x: trunc_f64(line.p2.x),
                y: trunc_f64(line.p2.y),
            },
        }
    };
    let mut lines = polygon
        .lines
        .clone()
        .iter()
        .map(|line| trunc_line(line))
        .collect::<Vec<Line>>();

    let mut new_lines = adjust_lines(width, height, &lines);
    let mut new_lines = union_lines(&new_lines);

    let mut new_lines = new_lines
        .into_iter()
        .filter(|line| line.p1.dist(&line.p2) > 0.0)
        .collect::<Vec<Line>>();

    // let mut new_lines = connect_line(&new_lines);

    // TODO 外枠との結合
    Polygon {
        point_id: polygon.point_id,
        lines: new_lines.clone(),
    }
}

/**
 * 範囲を超える直線を範囲内に収める
 */
fn adjust_lines(width: f64, height: f64, lines: &Vec<Line>) -> Vec<Line> {
    let adjust_value = |range: f64, value: f64| -> f64 {
        if value < 0.0 {
            0.0
        } else if value > range {
            range
        } else {
            value
        }
    };

    let mut new_lines: Vec<Line> = Vec::new();
    for i in 0..lines.len() {
        let line = &lines[i];
        let p1 = line.get_start_point();
        let p2 = line.get_end_point();
        if p1.x >= 0.0
            && p1.x <= width
            && p1.y >= 0.0
            && p1.y <= height
            && p2.x >= 0.0
            && p2.x <= width
            && p2.y >= 0.0
            && p2.y <= height
        {
            new_lines.push(line.clone());
            continue;
        }

        let (a, b, c) = line.get_factors();

        // x座標の調整
        let p1_x = adjust_value(width, p1.x);
        let p2_x = adjust_value(width, p2.x);
        let p1_y = (-c - a * p1_x) / b;
        let p2_y = (-c - a * p2_x) / b;

        // y座標の調整
        let p1_y = adjust_value(height, p1_y);
        let p2_y = adjust_value(height, p2_y);
        let p1_x = (-c - b * p1_y) / a;
        let p2_x = (-c - b * p2_y) / a;

        new_lines.push(Line {
            p1: Point {
                id: p1.id,
                x: trunc_f64(p1_x),
                y: trunc_f64(p1_y),
            },
            p2: Point {
                id: p2.id,
                x: trunc_f64(p2_x),
                y: trunc_f64(p2_y),
            },
        });
    }
    new_lines
}

fn union_lines(lines: &Vec<Line>) -> Vec<Line> {
    let same_line = |a: &Point, b: &Point, p: &Point| -> bool {
        let ab = Point {
            id: 0,
            x: trunc_f64(-a.x + b.x),
            y: trunc_f64(-a.y + b.y),
        };
        let ap = Point {
            id: 0,
            x: trunc_f64(-a.x + p.x),
            y: trunc_f64(-a.y + p.y),
        };
        let abap = ab.x * ap.x + ab.y * ap.y;
        let abs_ab = (ab.x.powf(2.0) + ab.y.powf(2.0)).sqrt();
        let abs_ap = (ap.x.powf(2.0) + ap.y.powf(2.0)).sqrt();
        let cos_sita = trunc_f64(abap / (abs_ab * abs_ap)).abs();
        cos_sita == 1.0
    };

    let must_far_points = |points: &Vec<Point>| -> (Point, Point) {
        let mut pair_indexes = vec![0; 2];
        let mut far_distance = 0.0;
        for i in 0..(points.len() - 1) {
            let p1 = &points[i];
            for j in (i + 1)..(points.len()) {
                let p2 = &points[j];
                let distance = p1.dist(p2);
                if far_distance < distance {
                    pair_indexes[0] = i;
                    pair_indexes[1] = j;
                    far_distance = distance;
                }
            }
        }
        (
            points[pair_indexes[0]].clone(),
            points[pair_indexes[1]].clone(),
        )
    };

    // 同一直線状に存在する線分を統合
    let mut new_lines: Vec<Line> = Vec::new();
    let mut removed_indexes: HashSet<usize> = HashSet::new();
    for i in 0..lines.len() {
        if removed_indexes.contains(&i) {
            continue;
        }
        let line = &lines[i];
        let mut a = line.p1.clone();
        let mut b = line.p2.clone();

        if i >= (lines.len() - 1) {
            new_lines.push(Line {
                p1: a.clone(),
                p2: b.clone(),
            });
            break;
        }
        for j in (i + 1)..lines.len() {
            let p1 = &lines[j].p1.clone();
            let p2 = &lines[j].p2.clone();
            if !same_line(&a, &b, &p1) || !same_line(&a, &b, &p2) {
                continue;
            }

            // update a, b
            let (p1, p2) = must_far_points(&vec![a.clone(), b.clone(), p1.clone(), p2.clone()]);
            a = p1;
            b = p2;
            removed_indexes.insert(j);
        }

        new_lines.push(Line {
            p1: a.clone(),
            p2: b.clone(),
        });
    }
    new_lines
}

/**
 * つながっていない直線を結ぶ
 */
fn connect_line(lines: &Vec<Line>) -> Vec<Line> {
    let mut new_lines: Vec<Line> = Vec::new();
    let mut eliminate_indexes: HashSet<usize> = HashSet::new();

    let get_near_line = |lines: &Vec<Line>,
                         point: &Point,
                         eliminate_indexes: &HashSet<usize>|
     -> Option<(usize, usize)> {
        let mut min_distance = f64::MAX;
        let mut result: Option<(usize, usize)> = None;
        for i in 0..lines.len() {
            if eliminate_indexes.contains(&i) {
                continue;
            }
            let (distance1, distance2) = (lines[i].p1.dist(&point), lines[i].p2.dist(&point));
            if min_distance > distance1 || min_distance > distance2 {
                if distance1 < distance2 {
                    result = Some((i, 1));
                    min_distance = distance1;
                } else {
                    result = Some((i, 2));
                    min_distance = distance2;
                }
            }
        }
        result
    };

    // 最も原点に近い点を持つ直線を抽出
    let mut origin = Point {
        id: 0,
        x: 0.0,
        y: 0.0,
    };
    let mut ignore = true;
    while let Some((line_index, point_index)) = get_near_line(&lines, &origin, &eliminate_indexes) {
        let line = if ignore {
            lines[line_index].clone()
        } else {
            let (p1, p2) = if point_index == 1 {
                (origin.clone(), lines[line_index].p2.clone())
            } else {
                (lines[line_index].p1.clone(), origin.clone())
            };
            Line { p1: p1, p2: p2 }
        };
        eprintln!(
            "connect line: ({:?}, {:?}) to ({:?}, {:?})",
            lines[line_index].p1, lines[line_index].p2, line.p1, line.p2
        );
        new_lines.push(line.clone());

        origin = if point_index == 1 {
            line.p2.clone()
        } else {
            line.p1.clone()
        };
        ignore = false;
        eliminate_indexes.insert(line_index);
    }
    new_lines
}

fn print_polygons(polygons: &Vec<Polygon>) {
    for i in 0..polygons.len() {
        let polygon = &polygons[i];
        polygon.print();
    }
}

/**
 * ダミーポイントの追加
 */
fn add_dummy_points(width: f64, height: f64, points: &Vec<Point>) -> Vec<Point> {
    let mut points = points.clone();
    points.insert(
        0,
        Point {
            id: 0,
            x: width / 2.0,
            y: -height,
        },
    );
    points.push(Point {
        id: 0,
        x: width / 2.0,
        y: height * 2.0,
    });
    points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    points
}

/**
 * サイトイベント用の準線郡作成
 * pointsはyの昇順となっている必要あり
 */
fn get_site_event_timing(last_event_timing: f64, points: &Vec<Point>) -> Vec<f64> {
    let mut events: Vec<f64> = vec![];
    for i in 0..points.len() {
        events.push(points[i].y);
    }
    events.push(last_event_timing);
    events
}

fn create_point_one_voronoi(width: f64, height: f64) -> Vec<Polygon> {
    let lines = create_rect_lines(&vec![
        Point {
            x: 0.0,
            y: 0.0,
            ..Default::default()
        },
        Point {
            x: width,
            y: 0.0,
            ..Default::default()
        },
        Point {
            x: width,
            y: height,
            ..Default::default()
        },
        Point {
            x: 0.0,
            y: height,
            ..Default::default()
        },
    ]);
    vec![Polygon {
        point_id: 0,
        lines: lines,
    }]
}

fn create_point_twe_voronoi(width: f64, height: f64, points: &Vec<Point>) -> Vec<Polygon> {
    let mut points = points.clone();
    points.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let delta_y = get_delta(points[1].y, points[0].y);
    let delta_x = get_delta(points[1].x, points[0].x);
    let split_line = if delta_y == 0.0 {
        eprintln!("縦線で分割");
        // 縦線
        let x = delta_x / 2.0 + min_f64(points[0].x, points[1].x);
        Line {
            p1: Point {
                x: x,
                y: 0.0,
                ..Default::default()
            },
            p2: Point {
                x: x,
                y: height,
                ..Default::default()
            },
        }
    } else if delta_x == 0.0 {
        eprintln!("横線で分割");
        // 横線
        let y = delta_y / 2.0 + min_f64(points[0].y, points[1].y);
        Line {
            p1: Point {
                x: 0.0,
                y: y,
                ..Default::default()
            },
            p2: Point {
                x: width,
                y: y,
                ..Default::default()
            },
        }
    } else {
        let center_y = delta_y / 2.0 + points[0].y;
        let center_x = delta_x / 2.0 + points[0].x;
        eprintln!("split line center: ({}, {})", center_x, center_y);
        let a = -1.0 / (delta_y / delta_x);
        let b = center_y - a * center_x;
        eprintln!("slope: {}, section: {}", a, b);
        // 右肩上がり
        let (min_y, max_y) = if a > 0.0 {
            eprintln!("right up");
            (max_f64(0.0, b), min_f64(height, a * width + b))
        } else {
            eprintln!("right down");
            (min_f64(height, b), max_f64(0.0, a * width + b))
        };
        let max_x = (max_y - b) / a;
        let min_x = (min_y - b) / a;
        // -0.0の対応
        let (min_x, min_y, max_x, max_y) = { (min_x.abs(), min_y.abs(), max_x.abs(), max_y.abs()) };

        let min_point = Point {
            x: min_x,
            y: min_y,
            ..Default::default()
        };
        let max_point = Point {
            x: max_x,
            y: max_y,
            ..Default::default()
        };
        eprintln!("min point: {:?}", min_point);
        eprintln!("max point: {:?}", max_point);
        Line {
            p1: min_point,
            p2: max_point,
        }
    };
    eprintln!("split line: {:?}", &split_line);
    let min_point = split_line.p1.clone();
    let max_point = split_line.p2.clone();
    let left_top = Point {
        x: 0.0,
        y: height,
        ..Default::default()
    };
    let left_bottom = Point {
        x: 0.0,
        y: 0.0,
        ..Default::default()
    };
    let right_top = Point {
        x: width,
        y: height,
        ..Default::default()
    };
    let right_bottom = Point {
        x: width,
        y: 0.0,
        ..Default::default()
    };
    let all_points = vec![left_top, left_bottom, right_top, right_bottom];

    let create_polygon_line =
        |min_point: &Point, max_point: &Point, use_points: &Vec<&Point>| -> Vec<Line> {
            eprintln!("create_polygon_line: {:?}", use_points);
            let mut indexes: Vec<usize> = Vec::new();
            // max_pointからの距離が近い準に並べる
            let mut x = max_point.x;
            let mut y = max_point.y;
            for i in 0..use_points.len() {
                let mut min_distance = f64::MAX;
                let mut min_distance_index = 0usize;
                for j in 0..use_points.len() {
                    if indexes.iter().any(|value| *value == j) {
                        continue;
                    }

                    let distance = get_distance(use_points[j].x, x, use_points[j].y, y);
                    if distance < min_distance {
                        min_distance = distance;
                        min_distance_index = j;
                    }
                }
                x = use_points[min_distance_index].x;
                y = use_points[min_distance_index].y;
                indexes.push(min_distance_index);
            }

            // min -> maxのラインが初期ライン
            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line {
                p1: min_point.clone(),
                p2: max_point.clone(),
            });
            let mut x = max_point.x;
            let mut y = max_point.y;

            for i in 0..indexes.len() {
                let index = indexes[i];
                let point = use_points[index];
                if x == point.x && y == point.y {
                    continue;
                }

                lines.push(Line {
                    p1: Point {
                        x: x,
                        y: y,
                        ..Default::default()
                    },
                    p2: point.clone(),
                });
                x = point.x;
                y = point.y;
            }

            // 最終ポイントからminへのラインを追加
            if x != min_point.x || y != min_point.y {
                lines.push(Line {
                    p1: Point {
                        x: x,
                        y: y,
                        ..Default::default()
                    },
                    p2: min_point.clone(),
                });
            }

            lines
        };

    // (min_x, min_y) -> (max_x, max_y)の左回り
    // 線分から右側(上)だけを抽出して構築
    let lines1 = create_polygon_line(
        &min_point,
        &max_point,
        &all_points
            .iter()
            .filter(|point| split_line.get_point_direction(point) != LinePointDirection::Left)
            .collect::<Vec<&Point>>(),
    );

    // (max_x, max_y) -> (min_x, min_y)の右回り
    // 線分から左側(下)だけを抽出して構築
    let lines2 = create_polygon_line(
        &min_point,
        &max_point,
        &all_points
            .iter()
            .filter(|point| split_line.get_point_direction(point) != LinePointDirection::Right)
            .collect::<Vec<&Point>>(),
    );

    vec![
        Polygon {
            point_id: points[0].id,
            lines: lines1,
        },
        Polygon {
            point_id: points[1].id,
            lines: lines2,
        },
    ]
}

fn create_triangle_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            p1: points[2].clone(),
            p2: points[0].clone(),
        },
    ]
}

fn create_rect_lines(points: &Vec<Point>) -> Vec<Line> {
    vec![
        Line {
            p1: points[0].clone(),
            p2: points[1].clone(),
        },
        Line {
            p1: points[1].clone(),
            p2: points[2].clone(),
        },
        Line {
            p1: points[2].clone(),
            p2: points[3].clone(),
        },
        Line {
            p1: points[3].clone(),
            p2: points[0].clone(),
        },
    ]
}

fn min_f64(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn get_distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let x_delta = get_delta(x1, x2);
    let y_delta = get_delta(y1, y2);
    ((x_delta * x_delta) + (y_delta * y_delta)).sqrt()
}

fn get_delta(a: f64, b: f64) -> f64 {
    a - b
}

// 3つの母点を通る円の中心を求める関数
fn get_circle_center(point: &Vec<Point>) -> Point {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 1.0;
    let mut d = 0.0;
    for i in 0..3 {
        let index1 = i % 3;
        let index2 = (i + 1) % 3;
        let index3 = (i + 2) % 3;
        a += point[index1].x * (point[index2].y - point[index3].y);
        b += point[i % 3].x.powf(2.0) * (point[index2].y - point[index3].y);
        c *= point[index2].y - point[index3].y;
        d += point[index1].x * (point[index2].y.powf(2.0) - point[index3].y.powf(2.0))
            - point[index2].x * point[(i + 2) % 3].x * (point[index2].x - point[index3].x);
    }
    Point {
        x: (b - c) / 2.0 / a,
        y: d / 2.0 / a,
        ..Default::default()
    }
}

/**
 * 交点計算
 * 左から見て母点index1の放物線と母点index2の放物線が交わる交点の位置を計算する関数
 * generating_points: Vec<Point>,
 * index1: usize, 左の母点
 * index2: usize, 右の母点
 * rho: f64,      準線の位置
 */
fn get_intersection(points: &Vec<Point>, index1: usize, index2: usize, rho: f64) -> Point {
    let mut intersect = Point::new();

    // 与えられた焦点（focus_x,focus_y）と準線y=rhoによる2次関数にxを与えたときのyの値
    // x 取得したい点のx座標
    // focus_x 2次関数の焦点のx座標
    // focus_y 2次関数の焦点のy座標
    // rho 準線の位置
    let quadratic_func = |x: f64, focus_x: f64, focus_y: f64, rho: f64| -> f64 {
        return trunc_f64(-(x - focus_x).powf(2.0) / 2.0 / (rho - focus_y) + (rho + focus_y) / 2.0);
    };

    let x1 = points[index1].x;
    let y1 = points[index1].y;
    let x2 = points[index2].x;
    let y2 = points[index2].y;

    let a = y2 - y1;
    let b = (rho - y1) * x2 - (rho - y2) * x1;
    let c =
        (rho - y1) * x2.powf(2.0) - (rho - y2) * x1.powf(2.0) + (y1 - y2) * (rho - y1) * (rho - y2);

    if trunc_f64(y1 - rho).abs() == 0.0 {
        intersect.x = trunc_f64(x1);
        intersect.y = quadratic_func(intersect.x, x2, y2, rho);
    } else if trunc_f64(y2 - rho) == 0.0 {
        intersect.x = trunc_f64(x2);
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else if trunc_f64(a) == 0.0 {
        intersect.x = trunc_f64(c / b / 2.0);
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else if index1 < index2 {
        intersect.x = trunc_f64((b - (b.powf(2.0) - a * c).sqrt()) / a);
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else {
        intersect.x = trunc_f64((b - (b.powf(2.0) - a * c).sqrt()) / a);
        intersect.y = quadratic_func(intersect.x, x2, y2, rho);
    }
    intersect
}

fn trunc_f64(value: f64) -> f64 {
    (value * 100.0).trunc() / 100.0
}

/**
 * 線分と交点を元にSVG文字列を生成
 * see: https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
 */
pub fn create_svg(width: f64, height: f64, points: &Vec<Point>, polygons: &Vec<Polygon>) -> String {
    let aspect_retio = (width / height) as i64;
    let svg_width = 600i64;
    let svg_height = (svg_width / aspect_retio) as i64;
    let n = 20i64;
    let margin = 10i64;
    let line_color = "#121212";
    let point_color = "#fc1212";

    let mut rng = thread_rng();
    let mut polygon_line_colors: Vec<String> = vec![];
    for i in 0..polygons.len() {
        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        polygon_line_colors.push(format!("#{:#04x}{:#04x}{:#04x}", r, g, b).replace("0x", ""));
    }
    let mut svg = Document::new()
        .set(
            "viewBox",
            (
                0,
                0,
                (svg_width + 2 * margin) as usize,
                (svg_height + 2 * margin) as usize,
            ),
        )
        .set("width", (svg_width + 2 * margin) as usize)
        .set("height", (svg_height + 2 * margin) as usize)
        .set("style", "background-color:#F2F3F5");

    // グラフの外枠
    svg = svg.add(
        SvgRectangle::new()
            .set("x", 10)
            .set("y", 10)
            .set("width", svg_width)
            .set("height", svg_height)
            .set("fill", "#F5F5F5")
            .set("stroke", line_color)
            .set("stroke-width", 3),
    );

    let scale_width = (svg_width as f64 / width).floor();
    let scale_height = (svg_height as f64 / height).floor();
    let change_coordinate =
        |x: f64, y: f64, scale_width: f64, scale_height: f64, margin: usize| -> (usize, usize) {
            (
                (x * scale_width) as usize + margin,
                (y * scale_height) as usize + margin,
            )
        };

    let graph_margin = margin as usize;
    let get_point_by_id = |id: u32, points: &Vec<Point>| -> Option<Point> {
        for i in 0..points.len() {
            if points[i].id == id {
                return Some(points[i].clone());
            }
        }
        None
    };
    for i in 0..polygons.len() {
        let color = &polygon_line_colors[i];
        let polygon = &polygons[i];
        for j in 0..polygon.lines.len() {
            // x: margin + (x * scale).floor()
            // y: margin + (y * scale).floor()
            let line = &polygon.lines[j];
            let (x1, y1) = change_coordinate(
                line.p1.x,
                line.p1.y,
                scale_width,
                scale_height,
                graph_margin,
            );
            let (x2, y2) = change_coordinate(
                line.p2.x,
                line.p2.y,
                scale_width,
                scale_height,
                graph_margin,
            );
            svg = svg.add(get_svg_line(x1, y1, x2, y2, &color));
        }

        if let Some(point) = get_point_by_id(polygon.point_id, &points) {
            let (x, y) =
                change_coordinate(point.x, point.y, scale_width, scale_height, graph_margin);
            svg = svg.add(get_svg_circle(x, y, &color));
            let (x, y) = (x - 10, y - 10);
            svg = svg.add(get_svg_text(x, y, &color, point.id.to_string()));
        }
    }
    svg.to_string()
}

fn get_svg_line(x1: usize, y1: usize, x2: usize, y2: usize, line_color: &str) -> SvgLine {
    SvgLine::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", line_color)
        .set("stroke-width", 3)
        .set("stroke-linecap", "round")
}

fn get_svg_text(x: usize, y: usize, color: &str, text: String) -> SvgText {
    SvgText::new(text)
        .set("x", x)
        .set("y", y)
        .set("stroke", "none")
        .set("fill", color)
}

fn get_svg_circle(x: usize, y: usize, color: &str) -> SvgCircle {
    SvgCircle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", 3)
        .set("fill", color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_voronoi_lines_point_one() {
        let width = 100.0;
        let height = 100.0;
        let points = vec![Point {
            x: 50.0,
            y: 50.0,
            ..Default::default()
        }];
        let actual = calc_voronoi_lines(width, height, &points);
        let expect = vec![Polygon {
            point_id: 0,
            lines: vec![
                Line {
                    p1: Point {
                        x: 0.0,
                        y: 0.0,
                        ..Default::default()
                    },
                    p2: Point {
                        x: width,
                        y: 0.0,
                        ..Default::default()
                    },
                },
                Line {
                    p1: Point {
                        x: width,
                        y: 0.0,
                        ..Default::default()
                    },
                    p2: Point {
                        x: width,
                        y: height,
                        ..Default::default()
                    },
                },
                Line {
                    p1: Point {
                        x: width,
                        y: height,
                        ..Default::default()
                    },
                    p2: Point {
                        x: 0.0,
                        y: height,
                        ..Default::default()
                    },
                },
                Line {
                    p1: Point {
                        x: 0.0,
                        y: height,
                        ..Default::default()
                    },
                    p2: Point {
                        x: 0.0,
                        y: 0.0,
                        ..Default::default()
                    },
                },
            ],
        }];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_horizontal_split() {
        // 横線のテスト
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 50.0,
                y: 10.0,
                ..Default::default()
            },
            Point {
                x: 50.0,
                y: 30.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let half_y = 20.0;
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: half_y,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: half_y,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: half_y,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: half_y,
                            ..Default::default()
                        },
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: half_y,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: half_y,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: half_y,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: half_y,
                            ..Default::default()
                        },
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_vertical_split() {
        // 縦線のテスト
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 10.0,
                y: 20.0,
                ..Default::default()
            },
            Point {
                x: 30.0,
                y: 20.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let half_x = 20.0;
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: Point {
                            x: half_x,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: half_x,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: half_x,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: half_x,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: Point {
                            x: half_x,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: half_x,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: half_x,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: half_x,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_split_up() {
        // 45°斜めの分割(右肩下がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 25.0,
                y: 25.0,
                ..Default::default()
            },
            Point {
                x: 75.0,
                y: 75.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point {
            x: 0.0,
            y: height,
            ..Default::default()
        };
        let max_point = Point {
            x: width,
            y: 0.0,
            ..Default::default()
        };
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_split_down() {
        // 45°斜めの分割(右肩上がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 25.0,
                y: 75.0,
                ..Default::default()
            },
            Point {
                x: 75.0,
                y: 25.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point {
            x: 0.0,
            y: 0.0,
            ..Default::default()
        };
        let max_point = Point {
            x: width,
            y: height,
            ..Default::default()
        };
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_right_down_01() {
        // 斜めの分割(右上・右肩下がり)
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 70.0,
                y: 80.0,
                ..Default::default()
            },
            Point {
                x: 90.0,
                y: 90.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point {
            x: 72.5,
            y: height,
            ..Default::default()
        };
        let max_point = Point {
            x: width,
            y: 45.0,
            ..Default::default()
        };
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_calc_voronoi_lines_point_two_diagonal_right_down_02() {
        // 斜めの分割(中央付近・右肩下がり・a=2.0)
        let width = 100.0;
        let height = 100.0;
        let points = vec![
            Point {
                x: 55.0,
                y: 60.0,
                ..Default::default()
            },
            Point {
                x: 45.0,
                y: 40.0,
                ..Default::default()
            },
        ];
        let actual = calc_voronoi_lines(width, height, &points);
        let min_point = Point {
            x: 0.0,
            y: 75.0,
            ..Default::default()
        };
        let max_point = Point {
            x: width,
            y: 25.0,
            ..Default::default()
        };
        let expect = vec![
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
            Polygon {
                point_id: 0,
                lines: vec![
                    Line {
                        p1: min_point.clone(),
                        p2: max_point.clone(),
                    },
                    Line {
                        p1: max_point.clone(),
                        p2: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: width,
                            y: height,
                            ..Default::default()
                        },
                        p2: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                    },
                    Line {
                        p1: Point {
                            x: 0.0,
                            y: height,
                            ..Default::default()
                        },
                        p2: min_point.clone(),
                    },
                ],
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_union_lines_01() {
        let lines = vec![
            Line {
                p1: Point {
                    id: 0,
                    x: 1.0,
                    y: 50.0,
                },
                p2: Point {
                    id: 0,
                    x: 30.0,
                    y: 50.0,
                },
            },
            Line {
                p1: Point {
                    id: 0,
                    x: 15.0,
                    y: 50.0,
                },
                p2: Point {
                    id: 0,
                    x: 80.0,
                    y: 50.0,
                },
            },
        ];
        let actual = union_lines(&lines);
        let expect = vec![Line {
            p1: Point {
                id: 0,
                x: 1.0,
                y: 50.0,
            },
            p2: Point {
                id: 0,
                x: 80.0,
                y: 50.0,
            },
        }];
        assert_eq!(actual, expect);
    }
}
