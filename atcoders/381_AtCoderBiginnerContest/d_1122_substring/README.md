# outline

## Vec<Option<u64>>に変換する戦法

Result: TLE

<details><summary>Code</summary>

```rust
fn main_logic<W: Write>(w: &mut W, n: u64, a: Vec<u64>) {
    let a1 = parse(&a);
    let a2 = parse2(&a);
    let max1 = get_max_length(&a1);
    let max2 = get_max_length(&a2);
    let max_length = max(max1, max2);
    writeln!(w, "{}", max_length * 2).unwrap();
}

fn get_max_length(a: &Vec<Option<u64>>, lower_length: usize) -> usize {
    eprintln!("{:?}", &a);
    let mut i = 0;
    let mut max_length = 0;
    loop {
        if i >= a.len() {
            break;
        }

        if a[i].is_some() && {
            let mut length = 0;
            let mut hs: HashSet<u64> = HashSet::new();
            hs.insert(a[i].unwrap());
            loop {
                length += 1;
                if i + length >= a.len() {
                    break;
                }
                if a[i + length].is_none() {
                    break;
                }
                let value = a[i + length].unwrap();
                if hs.contains(&value) {
                    break;
                }
                hs.insert(value);
            }
            if max_length < length {
                max_length = length;
            }
        }
        i += 1;
    }
    max_length
}

fn check_substring(a: &Vec<Option<u64>>) {

}

fn parse(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a.chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}

fn parse2(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a[1..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[1..]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}
```

</details>

## Vec<Option<u64>>に変換する + 最大部分文字列長保持戦法

最大文字列長を維持しつつやる

<details><summary>Code</sumamry>

```rust
use proconio::input;
use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    main_logic(&mut stdout, n, a);
    stdout.flush().unwrap();
}

fn main_logic<W: Write>(w: &mut W, _n: u64, a: Vec<u64>) {
    let a1 = parse(&a);
    let a2 = parse2(&a);
    let mut max_length = 0;
    max_length = get_max_length(&a1, max_length);
    max_length = get_max_length(&a2, max_length);
    writeln!(w, "{}", max_length * 2).unwrap();
}

fn get_max_length(a: &Vec<Option<u64>>, lower_length: usize) -> usize {
    let mut max_length = lower_length;
    let mut i = 0;
    loop {
        match head(&a, i) {
            None => {
                break;
            }
            Some(j) => {
                let next_none_index = head_none(&a, j + 1);
                if j + 1 >= next_none_index {
                    // 次の要素がNone or 末尾
                    if max_length == 0 {
                        max_length = 1;
                    }
                    i = j + 1;
                } else {
                    let last_some_index = next_none_index - 1;
                    if last_some_index - j > max_length {
                        let length = get_max_length_substring(&a[j..=last_some_index], max_length);
                        if length > max_length {
                            max_length = length;
                        }
                    }
                    i = next_none_index + 1;
                }
            }
        }
    }
    max_length
}

/**
 * a: 全てSome
 */
fn get_max_length_substring(a: &[Option<u64>], lower_limit: usize) -> usize {
    let a = a.iter().map(|value| value.unwrap()).collect::<Vec<u64>>();
    let mut max_length = 0;
    for i in 0..(a.len() - (lower_limit + 1)) {
        let mut length = 0;
        let mut hs: HashSet<u64> = HashSet::new();
        hs.insert(a[i]);
        loop {
            length += 1;
            if a.len() <= i + length {
                break;
            }
            let value = a[i + length];
            if hs.contains(&value) {
                break;
            }
            hs.insert(value);
        }
        if max_length < length {
            max_length = length;
        }
    }
    max_length
}

fn head(a: &Vec<Option<u64>>, first_index: usize) -> Option<usize> {
    let value = a[first_index..]
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_some())
        .next();
    if value.is_none() {
        None
    } else {
        Some(value.unwrap().0 + first_index)
    }
}

fn head_none(a: &Vec<Option<u64>>, first_index: usize) -> usize {
    let value = a[first_index..]
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_none())
        .next();
    if value.is_none() {
        a.len() - 1
    } else {
        value.unwrap().0 + first_index
    }
}

fn parse(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a.chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}

fn parse2(a: &Vec<u64>) -> Vec<Option<u64>> {
    if a.len() % 2 == 0 {
        a[1..a.len() - 1]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    } else {
        a[1..]
            .chunks(2)
            .map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<Vec<_>>()
    }
}
```

</details>