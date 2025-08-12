use aggregator::{NewsArticle, SocialPost, Summary, User};
use std::fmt::Display;
use std::fmt::Debug;

// Summary trait as a parameter.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Same as above, but using a generic type.
pub fn notify_equal_to_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


//pub fn notify_compare(item1: &impl Summary, item2: &impl Summary) {
    // This will not compile because the compiler cannot guarantee that
    // item1 and item2 are of the same type.
    // println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
//}
// Use Traits Bound Syntax to solve:
pub fn notify_compare<T: Summary>(item1: &T, item2: &T) {
    // This will compile because we use Traing Bound Syntax to 
    // let the compiler guarantee that item1 and item2 are of the same type.
    println!("Compare 2 news: \n 1. {} \n 2. {}", item1.summarize(), item2.summarize());
}


// + syntax to combine multiple traits.
pub fn notify_multiple(item: &(impl Summary + Display)) {
}
pub fn notify_multiple_equal<T: Summary + Display>(item: &T) {
}

// Instead of
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// }
// Use where clause:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug {
            0
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
    // However, we cannot return multiple types here.
}

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
    
    let user = User {
        username: String::from("Penguin"),
        userid: String::from("1"),
        birthdate: String::from("15/03/1990")
    };
    // No Summary trait implementation for User struct, so it uses the default
    // and CANNOT point to a specific author.
    println!("Summarize on user: {}", user.summarize());

    notify(&post);
    notify(&article);
    notify(&user);

    notify_equal_to_notify(&post);
    notify_equal_to_notify(&article);
    notify_equal_to_notify(&user);

    // FOR NOW, this will not compile:
    //notify_compare(&post, &article);
    //notify_compare(&article, &user);
    //notify_compare(&post, &user);

    let article2 = NewsArticle {
        headline: String::from("Penguins lost in the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins lost again in the NHL.",
        ),
    };
    notify_compare(&article, &article2);
}