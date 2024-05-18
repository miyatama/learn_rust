use proconio::input;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufWriter, Write};

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

fn main() {
    input! {
        n: u64,
        s: [String; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, s);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, _n: u64, s: Vec<String>) {
    let trie_nodes = create_trie_tree(&s);
    print_trie_tree(&trie_nodes);
    let mut result = 0;
    for i in 1..trie_nodes.len() {
        result += calc_combination(trie_nodes[i].sub_total, 2);
    }
    writeln!(w, "{}", result).unwrap();
}

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
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec!["ab".to_string(), "abc".to_string(), "arc".to_string()],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["4"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            11,
            vec![
                "ab".to_string(),
                "bb".to_string(),
                "aaa".to_string(),
                "bba".to_string(),
                "baba".to_string(),
                "babb".to_string(),
                "aaaba".to_string(),
                "aabbb".to_string(),
                "a".to_string(),
                "a".to_string(),
                "b".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["32"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic03() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec![
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["3"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic04() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["0"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic05() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            3,
            vec![
                "xxx".to_string(),
                "xxx".to_string(),
                "xxx".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["9"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic06() {
        let mut buff = Vec::<u8>::new();
        main_logic(
            &mut buff,
            2,
            vec![
                "xy".to_string(),
                "xz".to_string(),
            ],
        );

        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["1"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

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

    #[test]
    fn test_create_trie_tree03() {
        let inputs = vec!["a".to_string(), "a".to_string(), "a".to_string()];
        let actual = create_trie_tree(&inputs);
        let expect = vec![
            TrieNode {
                parent: 0,
                childs: vec![('a', 1)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: '\0',
                node_count: 0,
                sub_total: 3,
                terminate: false,
            },
            // 1
            TrieNode {
                parent: 0,
                childs: HashMap::new(),
                node_value: 'a',
                node_count: 3,
                sub_total: 3,
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

    #[test]
    fn test_create_trie_tree04() {
        let inputs = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let actual = create_trie_tree(&inputs);
        let expect = vec![
            TrieNode {
                parent: 0,
                childs: vec![('a', 1), ('d', 4), ('g', 7)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: '\0',
                node_count: 0,
                sub_total: 3,
                terminate: false,
            },
            // 1
            TrieNode {
                parent: 0,
                childs: vec![('b', 2)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'a',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 2
            TrieNode {
                parent: 1,
                childs: vec![('c', 3)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'b',
                node_count: 0,
                sub_total: 1,
                terminate: false,
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
                parent: 0,
                childs: vec![('e', 5)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'd',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 5
            TrieNode {
                parent: 4,
                childs: vec![('f', 6)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 6
            TrieNode {
                parent: 5,
                childs: HashMap::new(),
                node_value: 'f',
                node_count: 1,
                sub_total: 1,
                terminate: true,
            },
            // 7
            TrieNode {
                parent: 0,
                childs: vec![('h', 8)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'g',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 8
            TrieNode {
                parent: 7,
                childs: vec![('i', 9)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'h',
                node_count: 0,
                sub_total: 1,
                terminate: false,
            },
            // 9
            TrieNode {
                parent: 8,
                childs: HashMap::new(),
                node_value: 'i',
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
