struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother{x: self.x, y: other.y}
    }
}
pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    /*
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("largest: {}", largest);
    */
    // println!("largest: {}", largest_i32(number_list));
    println!("largest: {}", largest(number_list));

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    // println!("largest: {}", largest_i32(char_list));
    println!("largest: {}", largest(char_list));

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 1.0, y: 2.0};
    let p3 = PointAnother{x: 1, y: 2.0};
    let p4 = PointAnother{x: "hello", y: 'c'};
    let p5 = p3.mixup(p4);
    println!("p5: ({}, {})", p5.x, p5.y);
}


/*
fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
*/

// PartialOrd + Copy = 比較を許す物だけに絞る
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}