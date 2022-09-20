use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn default_summarize_call(&self) -> String {
        format!("call {}", self.summarize())
    }
}

pub struct NewsArticle {
    pub location: String,
    pub headline: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, at {}", self.headline, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    return 1;
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("content"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_dispaly(&self) {
        if self.x >= self.y {
            println!("x {}", self.x);
        } else {
            println!("y {}", self.y);
        }
    }
}

fn test(a: i32) -> bool {
    if a > 100 {
        return false;
    };

    let mut i = 0;
    loop {
        i = i + 1;

        if i == 5 {
            break;
        }
    }


    if a > 3 {
        true
    } else {
        false
    }


}

fn main() {

    let c = {true};

    println!("{}", c);

    println!("Hello, world!");

    let article = NewsArticle {
        location: String::from("Canada"),
        headline: String::from("Good"),
    };

    println!("{}", article.summarize());

    println!("{}", article.default_summarize());

    println!("{}", article.default_summarize_call());

    notify(&article);

    let tweet = return_summarizable();

    notify(&tweet);

    let a = 1;
    let b = 2;

    let x = Pair::new(a, b);

    x.cmp_dispaly();

    // let article2 = NewsArticle {
    //     location: String::from("Canada"),
    //     headline: String::from("Good"),
    // };

    // let y = Pair::new(article, article2);

    test(1);
}
