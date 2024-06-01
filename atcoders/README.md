# outline

対AtCoder用のロジックメモ

## テンプレート

入力 + 出力 + テストのテンプレート

<details><summary>Code</summary><div>

```toml
[dependencies]
proconio = "0.4.5"
```

```rust
use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, n: u64) {
    writeln!(w, "").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(&mut buff, 343);
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["343"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }
}
```

</div></details>

## 標準入力の受け取り(proconio非使用)

<details><summary>Code</summary><div>

```rust
fn main() {
    fn read_buffer() -> Option<u64> {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        let s = buffer.trim().chars().collect::<String>();
        if s.len() == 0 {
            return None;
        }
        Some(s.parse::<u64>().unwrap())
    }
    let mut a = Vec::new();
    loop {
        match read_buffer() {
            None => break,
            Some(val) => {
                a.push(val);
            }
        }
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, a);
    stdout.flush().unwrap();
}
```

</div></details>

## 整数計算

### 平方根

<details><summary>Code</summary><div>

```rust
fn calc_squrt(n: u64) -> Option<u64> {
    let mut factor = 0u64;
    while factor.pow(2) < n {
        factor += 1;
        let val = factor.pow(2);
        if val == n {
            return Some(factor);
        }
        if val > n {
            break;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_test01() {
        let actual = calc_squrt(4);
        let expect = Some(2);
        assert_eq!(expect, actual)
    }

    #[test]
    fn sqrt_test02() {
        let actual = calc_squrt(5);
        let expect = None;
        assert_eq!(expect, actual)
    }

    #[test]
    fn sqrt_test03() {
        let actual = calc_squrt(100);
        let expect = Some(10);
        assert_eq!(expect, actual)
    }
}
```

</div></details>

### 立方根

指定された数以下の最大の立方根

<details><summary>Code</summary><div>

```rust
fn calc_cbrt(n: u64) -> Option<u64> {
    let mut result = 0;
    for i in 0.. {
        let x = i * i * i;
        if x > n {
            break;
        }
        result = i;
    }
    Some(result)
}
```

</div></details>

### 最小公倍数と最大公約数

<details><summary>Code</summary><div>

```rust
// 最小公倍数
fn lcm(factor1: u64, factor2: u64) -> u64 {
    factor1 * factor2 / gcd(factor1, factor2)
}

// 最大公約数
fn gcd(factor1: u64, factor2: u64) -> u64 {
    if factor2 == 0 {
        return factor1;
    }
    gcd(factor2, factor1 % factor2)
}
```

</div></details>


### 階乗

<details><summary>Code</summary><div>

```rust
fn factorial(x: u64) -> u64 {
    if x <= 1 {
        return 1;
    }
    x * factorial(x - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_01() {
        let actual = factorial(3);
        let expect = 6;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_factorial_02() {
        let actual = factorial(7);
        let expect = 5040;
        assert_eq!(expect, actual);
    }
}
```

</div></details>

### 組み合わせと順列

<details><summary>Code</summary><div>

```rust
fn calc_combination(factor1: u32, factor2: u32) -> u32 {
    calc_permutation(factor1, factor2) / calc_factorial(factor2)
}

fn calc_permutation(factor1: u32, factor2: u32) -> u32 {
    if factor1 < factor2 {
        return 0;
    }
    let mut result = 1;
    for factor in 0..factor2 {
        result *= factor1 - factor;
    }
    result
}

fn calc_factorial(factor1: u32) -> u32 {
    if factor1 <= 2 {
        return 2;
    }
    let mut result = 1;
    for factor in 2..=factor1 {
        result *= factor
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_combination01() {
        let actual = calc_combination(2, 3);
        let expect = 0;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_combination02() {
        let actual = calc_combination(3, 3);
        let expect = 1;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_combination03() {
        let actual = calc_combination(7, 2);
        let expect = 21;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_permutation01() {
        let actual = calc_permutation(2, 3);
        let expect = 0;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_permutation02() {
        let actual = calc_permutation(5, 5);
        let expect = 120;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_permutation03() {
        let actual = calc_permutation(5, 2);
        let expect = 20;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_factorial01() {
        let actual = calc_factorial(2);
        let expect = 2;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_calc_factorial02() {
        let actual = calc_factorial(5);
        let expect = 120;
        assert_eq!(expect, actual);
    }
}
```

</div></details>

## 実数

### 配列から最小最大を取得

<details><summary>Code</summary><div>

```kotlin
let f_array = vec![1.0, 2.0, 0.5, 8.0, 3.3, 4.2];
let max = f_array.iter().fold(f64::MIN, |m, v| m.max(*v));
let min = f_array.iter().fold(f64::MAX, |m, v| m.min(*v));
eprintln!("max: {}, min: {}", max, min);
```

</div></details>

## 配列

### よく使うやつ　

| name | method |
| :---- | :----- |
| `Vec` | push() |
| `HashSet` | insert(), contains() |
| `HashMap` | insert(), get(), contains_key(), keys(), values() |
| `VecDeque` | push_front(), push_back(), pop_front(), pop_back(), `VecDeque::from(vec![])`, `VecDeque.iter().collect()` |
| `BinaryHeap` | push(), pop(), `BinaryHeap::from(vec![])` |

### Uniq + Sum

指定した数値以下の要素に対してUniq + Sum。

<details><summary>Code</summary><div>

```rust
fn main_logic<W: Write>(w: &mut W, n: usize, k: u64, a: Vec<u64>) {
    let uniq_set: HashSet<u64> = a.into_iter().filter(|val| val <= &k).collect();
    let summary = Vec::from_iter(uniq_set).iter().fold(0, |sum, i| sum + i);
    writeln!(w, "{}", summary).unwrap();
}
```

</div></details>

### Vec<String>から位置を取得

<details><summary>code</summary><div>

```rust
fn main() {
    let map = vec![
        "..#..".to_string(),
        ".S##.".to_string(),
        ".##T.".to_string(),
        ".....".to_string(),
    ];
    let (tx, ty) = map
        .iter()
        .enumerate()
        .filter(|(_, map)| map.contains("T"))
        .map(|(index, map)| (index, map.find("T").unwrap()))
        .next()
        .unwrap();
    println!("x: {}, y: {}", tx, ty);
}
```

</div></details>


### 文字グラフ - Trie Tree

<details><summary>Code</summary><div>

```rust
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
struct TrieNode {
    // 親ノードのインデックス
    parent: usize,
    // 後続ノードの文字とインデックス
    childs: HashMap<char, usize>,
    // 自身が保有する文字
    node_value: char,
    // 出現カウント
    node_count: u32,
    // 子ノードを含めた出現数
    sub_total: u32,
    // 単語の終端
    terminate: bool,
}

fn print_trie_tree(trie_nodes: &Vec<TrieNode>) {
    eprintln!("```mermaid");
    eprintln!("stateDiagram-v2");
    eprintln!("  direction LR;");

    eprintln!("  state \"●\" as v_0");
    for i in 1..trie_nodes.len() {
        let trie_node = &trie_nodes[i];
        eprintln!(
            "  state \"{}, {}({})\" as v_{}",
            trie_node.node_value, trie_node.sub_total, trie_node.node_count, i,
        );
    }
    for i in 0..trie_nodes.len() {
        let trie_node = &trie_nodes[i];
        for to_index in trie_node.childs.values() {
            println!("  v_{} --> v_{}", i, to_index);
        }
    }
    println!("```");
    println!("");
}

fn create_trie_tree(s: &Vec<String>) -> Vec<TrieNode> {
    let mut trie_nodes: Vec<TrieNode> = Vec::new();
    trie_nodes.push(TrieNode {
        parent: 0,
        childs: HashMap::new(),
        node_value: '\0',
        node_count: 0,
        sub_total: 0,
        terminate: false,
    });

    for i in 0..s.len() {
        let chars = s[i].chars().collect::<Vec<char>>().clone();
        let mut target_index = 0;
        for j in 0..chars.len() {
            let target_char = chars[j];
            let terminate = j == (chars.len() - 1);
            let node_count = if terminate { 1 } else { 0 };
            let child = &trie_nodes[target_index].childs.get(&target_char);
            match child {
                None => {
                    let index = trie_nodes.len();
                    let mut trie_node = trie_nodes[target_index].clone();
                    trie_node.childs.insert(chars[j], index);
                    trie_nodes[target_index] = trie_node;

                    trie_nodes.push(TrieNode {
                        parent: target_index,
                        childs: HashMap::new(),
                        node_value: chars[j],
                        node_count: node_count,
                        sub_total: 0,
                        terminate: terminate,
                    });
                    target_index = index;
                }
                Some(&next_index) => {
                    if terminate {
                        let mut trie_node = trie_nodes[next_index].clone();
                        trie_node.node_count += 1;
                        trie_nodes[next_index] = trie_node;
                    }
                    target_index = next_index;
                }
            }
        }
    }

    // 単語数の設定
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut summary: HashMap<usize, u32> = HashMap::new();
    let get_child_summary = |indexes: &Vec<usize>, summary: &HashMap<usize, u32>| -> Option<u32> {
        if indexes.len() <= 0 {
            return None;
        }
        let mut sum = 0;
        for i in 0..indexes.len() {
            match summary.get(&indexes[i]) {
                None => {
                    return None;
                }
                Some(value) => {
                    sum += value;
                }
            }
        }
        Some(sum)
    };
    queue.push_back(0);
    while let Some(index) = queue.pop_back() {
        let mut trie_node = trie_nodes[index].clone();
        let child_indexes = trie_node
            .childs
            .values()
            .map(|value| *value)
            .collect::<Vec<usize>>()
            .clone();
        match get_child_summary(&child_indexes, &summary) {
            // 子ノードの単語数が未設定
            None => {
                for i in 0..child_indexes.len() {
                    if !summary.contains_key(&child_indexes[i]) {
                        queue.push_back(child_indexes[i]);
                    }
                }

                // 子ノードが存在しない(末端のノード)
                if child_indexes.len() <= 0 {
                    trie_node.sub_total = trie_node.node_count;
                    trie_nodes[index] = trie_node.clone();
                    queue.push_back(trie_node.parent);
                    summary.insert(index, trie_node.sub_total);
                }
            }
            Some(sum) => {
                // 自身の単語数を更新して親のノードを追加
                trie_node.sub_total = sum + trie_node.node_count;
                trie_nodes[index] = trie_node.clone();
                // ルートの子要素が計算済みの場合は終了
                if index == 0 {
                    break;
                }
                summary.insert(index, trie_node.sub_total);
                queue.push_back(trie_node.parent);
            }
        }
    }
    trie_nodes.clone()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_trie_tree01() {
        let inputs = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "acc".to_string(),
            "ace".to_string(),
            "accept".to_string(),
            "exit".to_string(),
        ];
        let actual = create_trie_tree(&inputs);
        let expect = vec![
            TrieNode {
                parent: 0,
                childs: vec![('a', 1), ('e', 10)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: '\0',
                node_count: 0,
                sub_total: 7,
                terminate: false,
            },
            // 1
            TrieNode {
                parent: 0,
                childs: vec![('b', 2), ('c', 4)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'a',
                node_count: 1,
                sub_total: 6,
                terminate: true,
            },
            // 2
            TrieNode {
                parent: 1,
                childs: vec![('c', 3)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'b',
                node_count: 1,
                sub_total: 2,
                terminate: true,
            },
            // 3
            TrieNode {
                parent: 2,
                childs: HashMap::new(),
                node_value: 'c',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 4
            TrieNode {
                parent: 1,
                childs: vec![('c', 5), ('e', 6)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 0,
                sub_total: 3,
                terminate: false,
            },
            // 5
            TrieNode {
                parent: 4,
                childs: vec![('e', 7)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 1,
                sub_total: 2,
                terminate: true,
            },
            // 6
            TrieNode {
                parent: 4,
                childs: HashMap::new(),
                node_value: 'e',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 7
            TrieNode {
                parent: 5,
                childs: vec![('p', 8)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 8
            TrieNode {
                parent: 7,
                childs: vec![('t', 9)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'p',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 9
            TrieNode {
                parent: 8,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 10
            TrieNode {
                parent: 0,
                childs: vec![('x', 11)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 11
            TrieNode {
                parent: 10,
                childs: vec![('i', 12)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'x',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 12
            TrieNode {
                parent: 11,
                childs: vec![('t', 13)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'i',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 13
            TrieNode {
                parent: 12,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
        ];

        assert_eq!(expect.len(), actual.len());
        for i in 0..expect.len() {
            assert_eq!(expect[i].parent, actual[i].parent);
            assert_eq!(expect[i].childs.len(), actual[i].childs.len());
            for (key, value) in &expect[i].childs {
                let actual_value = actual[i].childs.get(&key);
                assert_eq!(true, actual_value.is_some());
                assert_eq!(*value, *actual_value.unwrap());
            }
            assert_eq!(expect[i].node_value, actual[i].node_value);
            assert_eq!(expect[i].node_count, actual[i].node_count);
            assert_eq!(expect[i].sub_total, actual[i].sub_total);
            assert_eq!(expect[i].terminate, actual[i].terminate);
        }
    }

    #[test]
    fn test_create_trie_tree02() {
        let inputs = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "acc".to_string(),
            "ace".to_string(),
            "accept".to_string(),
            "exit".to_string(),
            "a".to_string(),
        ];
        let actual = create_trie_tree(&inputs);
        let expect = vec![
            TrieNode {
                parent: 0,
                childs: vec![('a', 1), ('e', 10)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: '\0',
                node_count: 0,
                sub_total: 8,
                terminate: false,
            },
            // 1
            TrieNode {
                parent: 0,
                childs: vec![('b', 2), ('c', 4)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'a',
                node_count: 2,
                sub_total: 7,
                terminate: true,
            },
            // 2
            TrieNode {
                parent: 1,
                childs: vec![('c', 3)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'b',
                node_count: 1,
                sub_total: 2,
                terminate: true,
            },
            // 3
            TrieNode {
                parent: 2,
                childs: HashMap::new(),
                node_value: 'c',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 4
            TrieNode {
                parent: 1,
                childs: vec![('c', 5), ('e', 6)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 0,
                sub_total: 3,
                terminate: false,
            },
            // 5
            TrieNode {
                parent: 4,
                childs: vec![('e', 7)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 1,
                sub_total: 2,
                terminate: true,
            },
            // 6
            TrieNode {
                parent: 4,
                childs: HashMap::new(),
                node_value: 'e',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 7
            TrieNode {
                parent: 5,
                childs: vec![('p', 8)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 8
            TrieNode {
                parent: 7,
                childs: vec![('t', 9)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'p',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 9
            TrieNode {
                parent: 8,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 10
            TrieNode {
                parent: 0,
                childs: vec![('x', 11)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 11
            TrieNode {
                parent: 10,
                childs: vec![('i', 12)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'x',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 12
            TrieNode {
                parent: 11,
                childs: vec![('t', 13)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                node_value: 'i',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 13
            TrieNode {
                parent: 12,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
        ];

        assert_eq!(expect.len(), actual.len());
        for i in 0..expect.len() {
            eprintln!("expect[{}]: {:?}", i, expect[i]);
            eprintln!("actual[{}]: {:?}", i, actual[i]);
            assert_eq!(expect[i].parent, actual[i].parent);
            assert_eq!(expect[i].childs.len(), actual[i].childs.len());
            for (key, value) in &expect[i].childs {
                let actual_value = actual[i].childs.get(&key);
                assert_eq!(true, actual_value.is_some());
                assert_eq!(*value, *actual_value.unwrap());
            }
            assert_eq!(expect[i].node_value, actual[i].node_value);
            assert_eq!(expect[i].node_count, actual[i].node_count);
            assert_eq!(expect[i].sub_total, actual[i].sub_total);
            assert_eq!(expect[i].terminate, actual[i].terminate);
        }
    }
}
```

</div></details>

### ソート - マクロ

出典: [Rust の構造体Vecをsort_byしやすくするマクロを書いた](https://vraisamis.hatenadiary.jp/entry/2019/08/09/022201)

<details><summary>Code</summary><div>

呼び出し

```rust
// struct vec
v.sort_by(order_by!(x, y, z));
v.sort_by(order_by!(x asc, y desc));
v.sort_by(order_by!(z asc, x));

// tuple vec
v2.sort_by(order_by!(0, 1, 2));
v2.sort_by(order_by!(0 desc, 1, 2));
v2.sort_by(order_by!(2, 0, 1));
```


マクロ定義

```rust
macro_rules! order_by {
    ($($x:tt)*) => {
        |l, r| {
            order_by_inner!(l, r, $($x)*)
        }
    }
}
macro_rules! order_by_inner {
    () => {};
    ($l:ident) => {std::cmp::Ordering::Equal};
    ($l:ident , ) => {std::cmp::Ordering::Equal};
    ($l:ident , $r:ident) => {std::cmp::Ordering::Equal};
    ($l:ident , $r:ident , ) => {std::cmp::Ordering::Equal};

    // last
    ($l:ident , $r:ident , $x:tt asc) => {
        $l.$x.cmp(&$r.$x)
    };
    ($l:ident , $r:ident , $x:tt desc) => {
        $l.$x.cmp(&$r.$x).reverse()
    };
    ($l:ident , $r:ident , $x:tt) => {
        order_by_inner!($l, $r, $x asc)
    };

    // mid
    ($l:ident , $r:ident , $x:tt asc , $($p:tt)+) => {
        match $l.$x.cmp(&$r.$x) {
            std::cmp::Ordering::Equal => {
                order_by_inner!($l, $r, $($p)+)
            },
            other => other
        }
    };
    ($l:ident , $r:ident , $x:tt desc , $($p:tt)+) => {
        match $l.$x.cmp(&$r.$x).reverse() {
            std::cmp::Ordering::Equal => {
                order_by_inner!($l, $r, $($p)+)
            },
            other => other
        }
    };
    ($l:ident , $r:ident , $x:tt , $($p:tt)+) => {
        order_by_inner!($l, $r, $x asc, $($p)+)
    };
}
```

</div></details>

## 図形

### 三角形の範囲内判定

外積を使った範囲内判定

<details><summary>Code</summary><div>

```rust
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn in_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    eprintln!("is_triangle({:?}, {:?}, {:?}, {:?})", a, b, c, p);

    let ab = Point{x: b.x - a.x, y: b.y - a.y};
    let bc = Point{x: c.x - b.x, y: c.y - b.y};
    let ca = Point{x: a.x - c.x, y: a.y - c.y};

    let ap = Point{x: p.x - a.x, y: p.y - a.y};
    let bp = Point{x: p.x - b.x, y: p.y - b.y};
    let cp = Point{x: p.x - c.x, y: p.y - c.y};

    let c1 = ab.x * bp.y - ab.y * bp.x;
    let c2 = bc.x * cp.y - bc.y * cp.x;
    let c3 = ca.x * ap.y - ca.y * ap.x;

    (c1 > 0 && c2 > 0 && c3 > 0) ||
    (c1 < 0 && c2 < 0 && c3 < 0)
}
```

</div></details>