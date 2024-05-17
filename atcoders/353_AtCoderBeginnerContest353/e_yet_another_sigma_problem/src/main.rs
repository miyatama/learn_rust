use proconio::input;
use std::collections::HashMap;
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

fn main_logic<W: Write>(w: &mut W, n: u64, s: Vec<String>) {
    writeln!(w, "").unwrap();
}

fn create_trie_tree(s: &Vec<String>) -> Vec<TrieNode> {
    let mut trie_nodes: Vec<TrieNode> = Vec::new();
    for i in 0..s.len() {}
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
                childs: vec![('a', 1), ('e', 10)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: '\0',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 1
            TrieNode {
                parent: 0,
                childs: vec![('b', 2), ('c', 4)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'a',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 2
            TrieNode {
                parent: 1,
                childs: vec![('c', 3)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'b',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 3
            TrieNode {
                parent: 2,
                childs: HashMap::new(),
                node_value: 'c',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 4
            TrieNode {
                parent: 1,
                childs: vec![('c', 5), ('e', 6)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 5
            TrieNode {
                parent: 4,
                childs: vec![('e', 7)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'c',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 6
            TrieNode {
                parent: 4,
                childs: HashMap::new(),
                node_value: 'e',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 7
            TrieNode {
                parent: 5,
                childs: vec![('p', 8)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 8
            TrieNode {
                parent: 7,
                childs: vec![('t', 9)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'p',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 9
            TrieNode {
                parent: 8,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
            // 10
            TrieNode {
                parent: 0,
                childs: vec![('x', 10)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'e',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 11
            TrieNode {
                parent: 10,
                childs: vec![('i', 11)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'x',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 12
            TrieNode {
                parent: 11,
                childs: vec![('t', 13)].into_iter().collect::<HashMap<char, usize>>(),
                node_value: 'i',
                node_count: 0,
                sub_total: 0,
                terminate: false,
            },
            // 13
            TrieNode {
                parent: 12,
                childs: HashMap::new(),
                node_value: 't',
                node_count: 1,
                sub_total: 0,
                terminate: true,
            },
        ];

        assert_eq!(expect.len(), actual.len());
        for i in 0..expect.len() {
            assert_eq!(expect[i], actual[i]);
        }
    }
}
