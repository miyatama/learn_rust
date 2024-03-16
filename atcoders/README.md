# outline

対AtCoder用のロジックメモ

## テンプレート

入力 + 出力 + テストのテンプレート

<details><summary>Code</summary><div>

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