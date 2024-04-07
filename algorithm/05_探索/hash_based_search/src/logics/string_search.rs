use std::collections::HashMap;
use std::io::Write;

pub fn string_hash_based_search<W: Write>(w: &mut W, _n: u64, a: Vec<String>, value: String) {
    let result = hash_based_search(a, value);
    match result {
        None => writeln!(w, "Not Found").unwrap(),
        Some(_) => writeln!(w, "Found").unwrap(),
    }
}

fn hash_based_search(list: Vec<String>, value: String) -> Option<()> {
    let hs = load_table(list);

    let hash = calc_hash(&value);
    let vec = hs.get(&hash).unwrap();

    match vec.contains(&value) {
        true => Some(()),
        false => None,
    }
}

fn load_table(list: Vec<String>) -> HashMap<u64, Vec<String>> {
    let mut hs: HashMap<u64, Vec<String>> = HashMap::new();
    list.iter().for_each(|val| {
        let hash = calc_hash(val);
        let mut vec = match hs.get(&hash) {
            None => Vec::<String>::new(),
            Some(v) => v.to_vec(),
        };
        vec.push(val.to_string());
        hs.insert(hash, vec);
    });
    hs
}

fn calc_hash(s: &String) -> u64 {
    s.chars()
        .map(|c| c as u64)
        .fold(0, |sum, val| sum * 31 + val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic01() {
        let mut buff = Vec::<u8>::new();
        string_hash_based_search(&mut buff, 100, generate_word_vec(), "stand".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    #[test]
    fn test_main_logic02() {
        let mut buff = Vec::<u8>::new();
        string_hash_based_search(&mut buff, 100, generate_word_vec(), "stand".to_string());
        let actual = String::from_utf8(buff).unwrap();
        let actual = actual.split("\n").collect::<Vec<&str>>();
        let expect = vec!["Found"];
        (0..expect.len()).for_each(|index| {
            assert_eq!(expect[index], actual[index]);
        });
    }

    fn generate_word_vec() -> Vec<String> {
        vec![
            "play".to_string(),
            "government".to_string(),
            "run".to_string(),
            "small".to_string(),
            "number".to_string(),
            "off".to_string(),
            "always".to_string(),
            "move".to_string(),
            "like".to_string(),
            "night".to_string(),
            "live".to_string(),
            "Mr".to_string(),
            "point".to_string(),
            "believe".to_string(),
            "hold".to_string(),
            "today".to_string(),
            "bring".to_string(),
            "happen".to_string(),
            "next".to_string(),
            "without".to_string(),
            "before".to_string(),
            "large".to_string(),
            "all".to_string(),
            "million".to_string(),
            "must".to_string(),
            "home".to_string(),
            "under".to_string(),
            "water".to_string(),
            "room".to_string(),
            "write".to_string(),
            "mother".to_string(),
            "area".to_string(),
            "national".to_string(),
            "money".to_string(),
            "story".to_string(),
            "young".to_string(),
            "fact".to_string(),
            "month".to_string(),
            "different".to_string(),
            "lot".to_string(),
            "right".to_string(),
            "study".to_string(),
            "book".to_string(),
            "eye".to_string(),
            "job".to_string(),
            "word".to_string(),
            "though".to_string(),
            "business".to_string(),
            "issue".to_string(),
            "side".to_string(),
            "kind".to_string(),
            "four".to_string(),
            "head".to_string(),
            "far".to_string(),
            "black".to_string(),
            "long".to_string(),
            "both".to_string(),
            "little".to_string(),
            "house".to_string(),
            "yes".to_string(),
            "after".to_string(),
            "since".to_string(),
            "long".to_string(),
            "provide".to_string(),
            "service".to_string(),
            "around".to_string(),
            "friend".to_string(),
            "important".to_string(),
            "father".to_string(),
            "sit".to_string(),
            "away".to_string(),
            "until".to_string(),
            "power".to_string(),
            "hour".to_string(),
            "game".to_string(),
            "often".to_string(),
            "yet".to_string(),
            "line".to_string(),
            "political".to_string(),
            "end".to_string(),
            "among".to_string(),
            "ever".to_string(),
            "stand".to_string(),
            "bad".to_string(),
            "lose".to_string(),
            "however".to_string(),
            "member".to_string(),
            "pay".to_string(),
            "law".to_string(),
            "meet".to_string(),
            "car".to_string(),
            "city".to_string(),
            "almost".to_string(),
            "include".to_string(),
            "continue".to_string(),
            "set".to_string(),
            "later".to_string(),
            "community".to_string(),
            "much".to_string(),
            "name7".to_string(),
        ]
    }
}
