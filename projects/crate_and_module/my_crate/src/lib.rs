pub mod module_a;
mod module_b;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// some module in one file
/*
mod module_a {
    fn calc() {}
}

mod module_b{
    fn calc() {}
}
 */
