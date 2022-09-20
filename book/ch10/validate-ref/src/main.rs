use std::fmt::Display;
fn main() {
    println!("Hello, world!");

    let r;

    let x = 5;
    r = &x;

    //& wont compile 
    // {
    //     let x = 5;
    //     r = &x;
    // }

    println!("r: {}", r);

    let s = String::from("abs");
    let b = "cccd";

    let c = longest(&s, &b);

    println!("{}", c);

    let c = longestv2(&s, &b);

    println!("{}", c);

    let novel = String::from("call me");

    let i = ImportantExcerpt {
        part: &novel
    };

    println!("{}", i.part);

    println!("{}",i.announce_and_return_part(&novel));

    println!("{}", 
        longestv3(&"aaaaaaaaaaa", &"tttttttttt", &"hello")
    )

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longestv2<'a>(x: &'a str, y: &str) -> &'a str {
        println!("{}",y);
        x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.part
    }
}

fn longestv3<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display,
{
 println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

