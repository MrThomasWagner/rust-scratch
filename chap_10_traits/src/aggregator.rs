use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("NewsArticle summary: {}: {}", self.headline, self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}'s article.", self.author)
    }
}

#[derive(Debug)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("SocialPost summary: {}: {}", self.username, self.content)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("{item} summary! {}", item.summarize());
}

pub fn alternate_notify<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Alternate for {item} summary! {}", item.summarize());
}

pub fn summarizable() -> impl Summary + Display {
    NewsArticle {
        author: String::from("summarizable author"),
        headline: String::from("sumamrizable headline"),
        content: String::from("sumamrizable content"),
    }
}
