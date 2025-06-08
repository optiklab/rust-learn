pub trait Summary {
    // Without default implementation
    //fn summarize(&self) -> String;

    // With default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String {

        // If we don't specify default implementation,
        // the compiler will require that this method is implemented in
        // every type that implements the Summary trait, like User.
        String::from("(unknown author)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct User {
    pub username: String,
    pub userid: String,
    pub birthdate: String
}

// No specific implementation for User, so it uses the default.
impl Summary for User {}