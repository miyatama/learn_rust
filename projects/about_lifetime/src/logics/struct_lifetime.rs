#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'frieren> ImportantExcerpt<'frieren> {
    fn level() -> i32 {
        3
    }
    fn level_self(&self) -> i32 {
        3
    }
    fn get_part(&self) -> &str {
        self.part
    }
}

pub fn run_struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", &i);
}
