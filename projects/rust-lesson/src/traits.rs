trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("message...")
    }
}

struct NewArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewArticle {
     fn summarize(&self) -> String {
         format!("{} by {} {}", self.headline, self.author, self.location)
     }
}

impl Message for NewArticle {

}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username,self.content)
    }
}

pub fn run() {
    let apple = Apple{};
    let banana = Banana{};
    println!("apple price: {}, banana price: {}", apple.price(), banana.price());
    get_price(apple);
    get_price(banana);

    let tweet = Tweet{
        username: String::from("miyatama"),
        content: String::from("happy birthday"),
        reply: false,
        retweet: true,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewArticle {
        headline: String::from("miyatama news"),
        location: String::from("saga"),
        author: String::from("miyatama"),
        content: String::from("first commit"),
    };
    println!("new article: {}", article.summarize());
    notify(&article);
    notify_another(&article);
    // notify_another(&tweet);
}

fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news: {}", item.summarize());
    println!("Breaking message: {}", item.message());
}