/*
// ファイル未分割のコード
#[derive(Debug)]
struct ID {
    value: String,
}
impl ID {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct Node {
    id: ID,
    label: String,
}
impl Node {
    fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}
 */

// 単純にファイル分割
mod id;
mod node;
mod domain;

use domain::{entity::node2::Node2, value_object::id2::ID2};

fn main() {
    // main内定義
    // let node = Node::new(ID::new("1"), "Node 1");

    // 単純にファイル分割
    let node = node::Node::new(id::ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);

    // ディレクトリ分割
    let node = Node2::new(ID2::new("2"), "Node 2");

    println!("Hello, module: {:?}", node);
}

