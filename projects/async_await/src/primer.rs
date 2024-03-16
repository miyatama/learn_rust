use futures::executor::block_on;

pub fn run() {
    block_on(async_main());
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

struct Song {
    name: String,
}

async fn learn_song() -> Song {
    Song {
        name: "miyata".to_string(),
    }
}

async fn sing_song(song: Song) {
    println!("{}", song.name);
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("dance");
}
