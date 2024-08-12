pub fn factorial(n: u32) -> u32 {
    let mut n = n;
    let mut sum = 1;
    while n >= 1 {
        sum  = sum * n;
        n = n - 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

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