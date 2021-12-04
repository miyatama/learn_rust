#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self {width, height}
    }

    fn area(&self) {
        println!("area is {}", self.width * self.height);
    }

    // fn area(self) {
    //     println!("area is {}", self.width * self.height);
    // }
}

pub fn run(){
    let user1 = User{
        username: String::from("miyatama"),
        email: String::from("n.miyata080825@gmail.com"),
        sign_in_count: 0,
        active: false,
    };
    // let user2 = user1;
    // can not use user1
    // println!("user1: username: {}, email: {}, sign_in_count: {}, active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    let mut user1 = User{
        username: String::from("miyatama"),
        email: String::from("n.miyata080825@gmail.com"),
        sign_in_count: 0,
        active: false,
    };
    user1.email = String::from("002.n.miyatama080825@gmail.com");
    println!("{:?}", user1);
    println!("{:#?}", user1);

    let user2 = build_user(
        String::from("saito"),
        String::from("yu.saito@tnt.co.jp")
    );
    println!("{:#?}", user2);

    let rectangle1 = Rectangle::create(20, 20);
    println!("{:#?}", rectangle1);
    rectangle1.area();
    println!("{:#?}", rectangle1);
}

fn build_user(username: String, email: String) -> User {
    // User{
    //     username: username,
    //     email: email,
    //     sign_in_count: 0,
    //     active: false,
    // }

    User{
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}