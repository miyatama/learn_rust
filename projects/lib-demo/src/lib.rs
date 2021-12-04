mod generator;

pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("random u8 is {}", n);
}