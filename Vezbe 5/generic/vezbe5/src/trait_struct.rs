// definisanje osobina za strukture

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

//definisana osobina Summary, slicno interfejsu, metode mogu a i ne moraju da imaju definisano telo 
pub trait Summary {
    fn summarize(&self) -> String;
}


//implementacija osobine za konkretnu strukturu -> implementacija polimorfizma
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub fn trait_struct() {
    let t = Tweet{
        username: String::from("pera"),
        content: String::from("First tweet"),
        reply: false, 
        retweet: false
    };

    let na = NewsArticle{
        headline: String::from("News article"),
        location: String::from("News 1"),
        author: String::from("Pera P"),
        content: String::from("First article, news, bla bla, Rust"),
    };

    println!("{}", t.summarize());
    println!("{}", na.summarize());
}
