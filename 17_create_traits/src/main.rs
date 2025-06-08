use aggregator::{NewsArticle, SocialPost, Summary, User};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    
    // No specific implementation for User, so it uses the default.
    let user = User {
        username: String::from("Penguin!"),
        userid: String::from("1"),
        birthdate: String::from("15/03/1990")
    };
    println!("Summarize on user: {}", user.summarize());
}