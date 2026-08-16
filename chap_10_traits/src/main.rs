use crate::aggregator::{NewsArticle, SocialPost, Summary, alternate_notify, notify, summarizable};

mod aggregator;

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        content: String::from("... content of the article"),
        author: String::from("Stephen King"),
    };

    let post = SocialPost {
        username: String::from("username"),
        content: String::from(" ... social post content ... "),
    };

    println!("Hello!");
    println!("{article:?}");
    println!("{post:?}");

    println!("{}", article.summarize());
    println!("{}", post.summarize());

    notify(&article);
    alternate_notify(&article);

    notify(&summarizable());
}
