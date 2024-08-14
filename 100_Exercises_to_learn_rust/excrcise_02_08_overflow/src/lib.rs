pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result = multiply(result, i);
    }
    result
}

fn multiply(a: u32, b: u32) -> u32 {
    let a: u64 = a as u64;
    let b: u64 = b as u64;
    let max: u64 = u32::MAX as u64;
    if (a * b) >= max {
        return 2_192_834_560;
    }
    (a * b) as u32
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        // With the default dev profile, this will panic when you run `cargo test`
        // We want it to wrap around instead
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // A large number literal using underscores to improve readability!
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}