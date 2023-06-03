use traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        // is this some kind of inside joke?
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}
