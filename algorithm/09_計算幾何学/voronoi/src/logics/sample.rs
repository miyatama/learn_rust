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
    // add dummy point
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
    let mut intersections: Vec<f64> = Vec::new();
    intersections.push(0.0);
    intersections.push(1.0);
    intersections.push(1.0);
    intersections.push(0.0);

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

        // intersection_pos = new PVector[intersection_num];
        let intersection_pos = Point { x: 0.0, y: 0.0 };
        for i in 0..intersection_num {
            let index1 = intersections.get(2 * i);
            let index2 = intersections.get(2 * i + 1);
            let start = getIntersection(generating_points, index1, index2, previous_event_timing);
            let end = getIntersection(generating_points, index1, index2, present_event_timing);
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
                    if (generating_points.get(next_generating_point_index).x
                        - intersection_pos[i].x)
                        .abs()
                        < 0.001
                    {
                        insert_position = i;
                        medial_insert_flag = 1;
                        exception_index.push(next_generating_point_index);
                        break;
                    }
                    if generating_points.get(next_generating_point_index).x < intersection_pos[j].x
                    {
                        insert_position = i;
                        break;
                    }
                }
                // 新たな交点の位置を挿入
                if medial_insert_flag == 1 {
                    // 新しく追加される母点のx座標がある交点の位置のx座標が一致する場合
                    intersections.push(2 * insert_position + 1, next_generating_point_index);
                    intersections.push(2 * (insert_position + 1), next_generating_point_index);
                } else if insert_position == 0 {
                    // 挿入位置が一番左側の場合
                    let outer_index = intersections.get(0);
                    intersections.push(0, outer_index);
                    intersections.push(1, next_generating_point_index);
                    intersections.push(2, next_generating_point_index);
                    intersections.push(3, outer_index);
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
                    intersections.push(2 * insert_position, inner_index);
                    intersections.push(2 * insert_position + 1, next_generating_point_index);
                    intersections.push(2 * (insert_position + 1), next_generating_point_index);
                    intersections.push(2 * (insert_position + 1) + 1, outer_index);
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
                    circle_point[k] = generating_points.get(circle_point_index[k]).clone();
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
// ボロノイ図を描く関数
void drawVoronoiDiagram ( ArrayList<PVector> generating_points) {

  generating_points.add(0, new PVector(width/2.0, -height)); // 母点にダミーの点を入れる
  generating_points.add(new PVector(width/2.0, 2.0*height)); // 母点にダミーの点を入れる

  // 母点を上から順番に並び変える
  generating_points.sort(new CompareToY());
  println(generating_points);

  // 最後のイベントタイミング（ダミー）を準備する。
  float last_event_timing = 2.0*height;

  // 汀線上の交点の初期化（ダミーを除く、一番上の母点の位置からスタート）
  ArrayList<Integer> intersections = new ArrayList<Integer>();
  intersections.add(0);
  intersections.add(1);
  intersections.add(1);
  intersections.add(0);

  println(intersections);

  // Site Eventが起こるタイミング
  ArrayList<Float> site_event_timing = new ArrayList<Float>();
  for(int i=0; i<generating_points.size(); i++){
    site_event_timing.add(generating_points.get(i).y);
  }
  site_event_timing.add(last_event_timing); // 最後のタイミング（ダミー）をsite_event_timingの最後に入れておく

  int site_event_number = 2; // 次のSite Eventで加えられる母点の番号を表す変数
  float present_event_timing = site_event_timing.get(site_event_number); // サイトイベント、サークルイベントを合わせた、現在のイベントのタイミング
  float previous_event_timing = site_event_timing.get(site_event_number-1); // // サイトイベント、サークルイベントを合わせた、直前のイベントのタイミング
  int event_type = 0; // サイトイベントのとき0、サークルイベントのとき1
  int intersection_num = 0;

  int next_generating_point_index = 2;
  PVector start, end;
  PVector[] intersection_pos;
  float next_circle_event_timing = last_event_timing;
  ArrayList<Integer> exception_index = new ArrayList<Integer>();

  while( present_event_timing < last_event_timing ){
    println("イベントタイミング:", present_event_timing);
    intersection_num = intersections.size()/2; // 汀線上の交点の数

    // 前のタイミングと現在のタイミングで交点の位置を取得してそれらの点を結んで境界線を描く
    intersection_pos = new PVector[intersection_num];
    for(int j=0; j<intersection_num; j++){
      int index1 = intersections.get(2*j);
      int index2 = intersections.get(2*j+1);
      start = getIntersection(generating_points, index1, index2, previous_event_timing);
      end = getIntersection(generating_points, index1, index2, present_event_timing);
      line(start.x, start.y, end.x, end.y);

      intersection_pos[j] = end.copy(); // 交点位置の更新のため、交点位置を保持しておく
    }

    // intersectionsの更新
    if( event_type == 0 ){ // site eventの場合
      // 新たに発生する交点の挿入位置を決める
      int medial_insert_flag = 0; // 新しく追加される母点のx座標がある交点の位置のx座標が一致する場合1、一致しない場合0
      int insert_position = intersection_num;
      for(int j=0; j<intersection_num; j++){
        if( abs( generating_points.get(next_generating_point_index).x - intersection_pos[j].x ) < 0.001){
          insert_position = j;
          medial_insert_flag = 1;
          exception_index.add(next_generating_point_index);
          break;
        }
        if( generating_points.get(next_generating_point_index).x < intersection_pos[j].x){
          insert_position = j;
          break;
        }
      }

      // 新たな交点の位置を挿入
      if( medial_insert_flag == 1 ){ // 新しく追加される母点のx座標がある交点の位置のx座標が一致する場合
        intersections.add(2*insert_position+1, next_generating_point_index);
        intersections.add(2*(insert_position+1), next_generating_point_index);
      } else if( insert_position == 0 ){ // 挿入位置が一番左側の場合
        int outer_index = intersections.get(0);
        intersections.add(0, outer_index);
        intersections.add(1, next_generating_point_index);
        intersections.add(2, next_generating_point_index);
        intersections.add(3, outer_index);
      } else if( insert_position == intersection_num ){ // 挿入位置が一番右側の場合
        int inner_index = intersections.get(2*(insert_position-1)+1);
        intersections.add(inner_index);
        intersections.add(next_generating_point_index);
        intersections.add(next_generating_point_index);
        intersections.add(inner_index);
      } else { // 挿入位置が配列の位置の間
        int inner_index = intersections.get(2*(insert_position-1)+1);
        int outer_index = intersections.get(2*insert_position);
        intersections.add(2*insert_position, inner_index);
        intersections.add(2*insert_position+1, next_generating_point_index);
        intersections.add(2*(insert_position+1), next_generating_point_index);
        intersections.add(2*(insert_position+1)+1, outer_index);
      }
      // 次の母点にindexを更新
      next_generating_point_index++;
      // 汀線上の交点の数を更新
      intersection_num++;

    } else { // サークルイベントの場合
      // 隣り合う交点の位置が重なった場合、交点同士を合体させる
      ArrayList<Integer> remove_index = new ArrayList<Integer>();
      for(int j=1; j<intersection_num; j++){
        if( exception_index.indexOf(intersections.get(2*j)) == -1 && intersections.get(2*j+1) != intersections.get(2*(j-1)) && intersection_pos[j].dist(intersection_pos[j-1]) < 2.0 ){
          remove_index.add(j);
        }
      }
      println("除外点:", remove_index);
      int remove_num = remove_index.size();
      for(int j=remove_num-1; j>=0; j--){
        intersections.remove(2*remove_index.get(j));
        intersections.remove(2*(remove_index.get(j)-1)+1);
        intersection_num--;
      }
    }

    // 次のcircleイベントのタイミングを算出する
    next_circle_event_timing = last_event_timing;
    for(int j=1; j<intersection_num; j++){
      // 3つの母点のindexを取得
      int[] circle_point_index = new int[3];
      circle_point_index[0] = intersections.get(2*(j-1));
      circle_point_index[1] = intersections.get(2*j);
      circle_point_index[2] = intersections.get(2*j+1);
      // 選択された隣り合った2つの交点のindexが異なる場合計算する
      if( circle_point_index[0] != circle_point_index[2] ){
        // 選択された3点を通る円の中心と半径を算出する
        PVector circle_center;
        float circle_radius;
        // 関数を挿入
        PVector[] circle_point = new PVector[3];
        for(int k=0; k<3; k++){
          circle_point[k] = generating_points.get(circle_point_index[k]).copy();
        }
        circle_center = getCercleCenter(circle_point);
        circle_radius = circle_point[0].dist(circle_center.copy());
        // 「円の中心のy座標＋半径」の値が現状のイベントタイミング以上かつ次のサークルイベント発生タイミングより小さい値であれば、
        // 次のサークルイベント発生タイミングを「円の中心のy座標＋半径」の値に更新する
        if( event_type == 0 && exception_index.indexOf(circle_point_index[1]) == -1 && (circle_center.y + circle_radius) > present_event_timing - 0.001 && circle_center.y + circle_radius <= next_circle_event_timing ){
          next_circle_event_timing = circle_center.y + circle_radius;
        } else if( event_type == 1 && present_event_timing < circle_center.y + circle_radius && circle_center.y + circle_radius < next_circle_event_timing ){
          next_circle_event_timing = circle_center.y + circle_radius;
        } else {
        }
      }
    }
    println("exception:", exception_index);

    // 次のタイミングにpresent_event_timingを更新
    previous_event_timing = present_event_timing;
    if( next_circle_event_timing >= site_event_timing.get(next_generating_point_index) ){
      present_event_timing = site_event_timing.get(next_generating_point_index);
      event_type = 0;
    } else {
      present_event_timing = next_circle_event_timing;
      event_type = 1;
    }
    if( abs( present_event_timing - previous_event_timing ) > 0.001 ){
      exception_index.clear();
    }

    println(intersections);

  }

  // 最後に残った境界線を描く
  for(int j=0; j<intersection_num; j++){
    int index1 = intersections.get(2*j);
    int index2 = intersections.get(2*j+1);
    start = getIntersection(generating_points, index1, index2, (float)previous_event_timing);
    end = getIntersection(generating_points, index1, index2, (float)present_event_timing);
    line(start.x, start.y, end.x, end.y);
  }

  // 母点を描く
  strokeWeight(10);
  for(int i=0; i<generating_points.size(); i++){
    point(generating_points.get(i).x,generating_points.get(i).y);
  }

  println("finish");
}

// 母点をそれらのyの値が小さい順でソートするためのクラス
class CompareToY implements Comparator<PVector>
{
  //@Override
  int compare(PVector v1, PVector v2)
  {
    return int(v1.y - v2.y);
  }
}

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

// 与えられた焦点（focus_x,focus_y）と準線y=rhoによる2次関数にxを与えたときのyの値
float quadratic_func(
  float x, // 取得したい点のx座標
  float focus_x, // 2次関数の焦点のx座標
  float focus_y, // 2次関数の焦点のy座標
  float rho // 準線の位置
){
  float y;
  y = - pow(x-focus_x,2)/2.0/(rho-focus_y) + (rho + focus_y)/2.0;
  return y;
}

// 左から見て母点index1の放物線と母点index2の放物線が交わる交点の位置を計算する関数
PVector getIntersection (
  ArrayList<PVector> generating_points,
  int index1, // 左の母点
  int index2, // 右の母点
  float rho // 準線の位置
){
  PVector intersect = new PVector(0.0,0.0);

  float x1 = (float) generating_points.get(index1).x;
  float y1 = (float) generating_points.get(index1).y;
  float x2 = (float) generating_points.get(index2).x;
  float y2 = (float) generating_points.get(index2).y;

  float a = y2 - y1;
  float b = (rho-y1)*x2-(rho-y2)*x1;
  float c = (rho-y1)*pow(x2,2)-(rho-y2)*pow(x1,2)+(y1-y2)*(rho-y1)*(rho-y2);

  if( abs(y1 - rho) < 0.001 ){
    intersect.x = x1;
    intersect.y = quadratic_func(intersect.x, x2, y2, rho);
  } else if( abs(y2 - rho) < 0.001 ){
    intersect.x = x2;
    intersect.y = quadratic_func(intersect.x, x1, y1, rho);
  } else if( abs(a) < 0.001 ){
    intersect.x = c/b/2.0;
    intersect.y = quadratic_func(intersect.x, x1, y1, rho);
  } else if ( index1 < index2 ){
    intersect.x = (b-sqrt(pow(b,2)-a*c))/a;
    intersect.y = quadratic_func(intersect.x, x1, y1, rho);
  } else {
    intersect.x = (b-sqrt(pow(b,2)-a*c))/a;
    intersect.y = quadratic_func(intersect.x, x2, y2, rho);
  }

  return intersect;

}
 */
pub fn get_points() -> Vec<Point> {
    return vec![Point { x: 0.0, y: 0.0 }];
}
