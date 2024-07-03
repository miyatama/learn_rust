#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

pub fn run() {
    // 母点集合
    let points = get_points();
    let height = 100.0;
    let width = 100.0;
    draw_voronoi_diagram(height, width, &points);
}

fn draw_voronoi_diagram(height: f64, width: f64, points: &Vec<Point>) {
    // 母点にダミーの点を入れる
    let mut points = points.clone();
    points.insert(
        0,
        Point {
            x: width / 2.0,
            y: -height,
        },
    );
    points.push(Point {
        x: width / 2.0,
        y: height * 2.0,
    });

    // 1: y降順
    // 2: x昇順
    points.sort();

    let last_event_timing = 2.0 * height;
    // 汀線の状態を保持する。ポイントIDを汀線順に並べる。
    let mut intersections: Vec<u32> = Vec::new();
    intersections.push(0);
    intersections.push(1);
    intersections.push(1);
    intersections.push(0);

    let mut site_event_timing: Vec<f64> = Vec::new();
    for i in 0..points.len() {
        site_event_timing.push(points[i].y);
    }
    site_event_timing.push(last_event_timing);

    let mut site_event_number = 2;
    let mut present_event_timing = site_event_timing.get(site_event_number);
    let mut previous_event_timing = site_event_timing.get(site_event_number - 1);
    let mut event_type = 0;
    let mut intersection_num = 0;

    let mut next_generating_point_index = 2;
    let mut start: Point = Point { x: 0.0, y: 0.0 };
    let mut end: Point = Point { x: 0.0, y: 0.0 };
    let mut intersection_pos: Vec<Point> = Vec::new();
    let mut next_circle_event_timing = last_event_timing;
    let mut exception_index: Vec<usize> = Vec::new();

    while present_event_timing < last_event_timing {
        println!("event timing: {}", present_event_timing);
        let intersection_num = intersections.size() / 2;

        let mut intersection_pos: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }; intersection_num];
        // 隣り合う交点
        for i in 0..intersection_num {
            let index1 = intersections.get(2 * i);
            let index2 = intersections.get(2 * i + 1);
            let start = getIntersection(points, index1, index2, previous_event_timing);
            let end = getIntersection(points, index1, index2, present_event_timing);
            // line(start.x, start.y, end.x, end.y);

            intersection_pos[i] = end.copy(); // 交点位置の更新のため、交点位置を保持しておく
        }

        // intersectionsの更新
        match event_type {
            0 => {
                println!("site event");
                let mut medial_insert_flag = 0;
                let mut insert_position = intersection_num;
                for i in 0..intersection_num {
                    if (points.get(next_generating_point_index).x - intersection_pos[i].x).abs()
                        < 0.001
                    {
                        insert_position = i;
                        medial_insert_flag = 1;
                        exception_index.push(next_generating_point_index);
                        break;
                    }
                    if points.get(next_generating_point_index).x < intersection_pos[j].x {
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
                    let outer_index = intersections.get(0);
                    intersections.insert(0, outer_index);
                    intersections.insert(1, next_generating_point_index);
                    intersections.insert(2, next_generating_point_index);
                    intersections.insert(3, outer_index);
                } else if insert_position == intersection_num {
                    // 挿入位置が一番右側の場合
                    let inner_index = intersections.get(2 * (insert_position - 1) + 1);
                    intersections.push(inner_index);
                    intersections.push(next_generating_point_index);
                    intersections.push(next_generating_point_index);
                    intersections.push(inner_index);
                } else {
                    // 挿入位置が配列の位置の間
                    let inner_index = intersections.get(2 * (insert_position - 1) + 1);
                    let outer_index = intersections.get(2 * insert_position);
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
            _ => {
                println!("circle event");
                // 隣り合う交点の位置が重なった場合、交点同士を合体させる
                let mut remove_index: Vec<usize> = Vec::new();
                for i in 1..intersection_num {
                    if exception_index.indexOf(intersections.get(2 * i)) == -1
                        && intersections.get(2 * j + 1) != intersections.get(2 * (i - 1))
                        && intersection_pos[j].dist(intersection_pos[i - 1]) < 2.0
                    {
                        remove_index.push(i);
                    }
                }
                println!("除外点:", remove_index);
                let remove_num = remove_index.size();
                for i in (0..(remove_num - 1)).rev() {
                    intersections.remove(2 * remove_index.get(i));
                    intersections.remove(2 * (remove_index.get(i) - 1) + 1);
                    intersection_num -= 1;
                }
            }
        }
        println!("calculate next circle event timing");

        next_circle_event_timing = last_event_timing;
        for j in 1..intersection_num {
            // 3つの母点のindexを取得
            let mut circle_point_index = vec![0; 3];
            circle_point_index[0] = intersections.get(2 * (j - 1));
            circle_point_index[1] = intersections.get(2 * j);
            circle_point_index[2] = intersections.get(2 * j + 1);
            // 選択された隣り合った2つの交点のindexが異なる場合計算する
            if circle_point_index[0] != circle_point_index[2] {
                // 選択された3点を通る円の中心と半径を算出する
                let mut circle_center: Point = Point {};
                let mut circle_radius = 0.0;
                // 関数を挿入
                let mut circle_point: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }; 3];
                for k in 0..3 {
                    circle_point[k] = points.get(circle_point_index[k]).clone();
                }
                circle_center = getCercleCenter(circle_point);
                circle_radius = circle_point[0].dist(circle_center.copy());
                // 「円の中心のy座標＋半径」の値が現状のイベントタイミング以上かつ次のサークルイベント発生タイミングより小さい値であれば、
                // 次のサークルイベント発生タイミングを「円の中心のy座標＋半径」の値に更新する
                if event_type == 0
                    && exception_index.indexOf(circle_point_index[1]) == -1
                    && (circle_center.y + circle_radius) > present_event_timing - 0.001
                    && circle_center.y + circle_radius <= next_circle_event_timing
                {
                    next_circle_event_timing = circle_center.y + circle_radius;
                } else if event_type == 1
                    && present_event_timing < circle_center.y + circle_radius
                    && circle_center.y + circle_radius < next_circle_event_timing
                {
                    next_circle_event_timing = circle_center.y + circle_radius;
                } else {
                }
            }
        }

        println!("calculate present event timing");
        previous_event_timing = present_event_timing;
        if next_circle_event_timing >= site_event_timing.get(next_generating_point_index) {
            present_event_timing = site_event_timing.get(next_generating_point_index);
            event_type = 0;
        } else {
            present_event_timing = next_circle_event_timing;
            event_type = 1;
        }
        if (present_event_timing - previous_event_timing).abs() > 0.001 {
            exception_index.clear();
        }
    }
}

/**
// 3つの母点を通る円の中心を求める関数
PVector getCercleCenter(
  PVector[] point
){
  PVector center = new PVector();
  float a = 0.0;
  float b = 0.0;
  float c = 1.0;
  float d = 0.0;
  for(int i=0; i<3; i++){
    a += point[i%3].x * ( point[(i+1)%3].y - point[(i+2)%3].y );
    b += pow(point[i%3].x,2) * ( point[(i+1)%3].y - point[(i+2)%3].y );
    c *= point[(i+1)%3].y - point[(i+2)%3].y;
    d += point[i%3].x * ( pow(point[(i+1)%3].y,2) - pow(point[(i+2)%3].y,2) ) - point[(i+1)%3].x * point[(i+2)%3].x * (point[(i+1)%3].x - point[(i+2)%3].x);
  }
  center.x = (b-c)/2.0/a;
  center.y = d/2.0/a;

  return center;
}
 */

pub fn get_points() -> Vec<Point> {
    return vec![Point { x: 0.0, y: 0.0 }];
}

/**
 * 交点計算
 * 左から見て母点index1の放物線と母点index2の放物線が交わる交点の位置を計算する関数
 * generating_points: Vec<Point>,
 * index1: usize, 左の母点
 * index2: usize, 右の母点
 * rho: f64,      準線の位置
 */
fn getIntersection(points: Vec<Point>, index1: usize, index2: usize, rho: f64) -> Point {
    let mut intersect = Ponint { x: 0.0, y: 0.0 };

    // 与えられた焦点（focus_x,focus_y）と準線y=rhoによる2次関数にxを与えたときのyの値
    // x 取得したい点のx座標
    // focus_x 2次関数の焦点のx座標
    // focus_y 2次関数の焦点のy座標
    // rho 準線の位置
    let quadratic_func = |x: f64, focus_x: f64, focus_y: f64, rho: f64| -> f64 {
        return -pow(x - focus_x, 2) / 2.0 / (rho - focus_y) + (rho + focus_y) / 2.0;
    };

    let x1 = points.get(index1).x;
    let y1 = points.get(index1).y;
    let x2 = points.get(index2).x;
    let y2 = points.get(index2).y;

    let a = y2 - y1;
    let b = (rho - y1) * x2 - (rho - y2) * x1;
    let c = (rho - y1) * pow(x2, 2) - (rho - y2) * pow(x1, 2) + (y1 - y2) * (rho - y1) * (rho - y2);

    if ((y1 - rho).abs() < 0.001) {
        intersect.x = x1;
        intersect.y = quadratic_func(intersect.x, x2, y2, rho);
    } else if ((y2 - rho).abs() < 0.001) {
        intersect.x = x2;
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else if (a.abs() < 0.001) {
        intersect.x = c / b / 2.0;
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else if (index1 < index2) {
        intersect.x = (b - sqrt(pow(b, 2) - a * c)) / a;
        intersect.y = quadratic_func(intersect.x, x1, y1, rho);
    } else {
        intersect.x = (b - sqrt(pow(b, 2) - a * c)) / a;
        intersect.y = quadratic_func(intersect.x, x2, y2, rho);
    }
    intersect
}
