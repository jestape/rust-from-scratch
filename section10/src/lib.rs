

use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


pub trait Summary {
    
    fn summarize_author(&self) -> String;
    fn summarize_content(&self) -> String;

    fn summarize(&self) -> String {
       format!("{}: {}", self.summarize_author(), self.summarize_content())
    }
}

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize_content(&self) -> String {
       format!("@{}", self.content)
    }

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize_content(&self) -> String {
        format!("@{}", self.content)
    }


}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
