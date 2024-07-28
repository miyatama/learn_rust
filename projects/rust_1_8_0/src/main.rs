use std::sync::LazyLock;
use std::time::Instant;

fn main() {
    let my_sales = 10000;
    println!("{} class yen", size_prefix(my_sales));
}

fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 10u32.pow(3);
    const M: u32 = 10u32.pow(6);
    const G: u32 = 10u32.pow(9);
    match n {
        ..K => "",
        K..M => "k",
        M..G => "M",
        G.. => "G",
    }
}

#[cfg(miyatama)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(true, false);
    }
}