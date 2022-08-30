fn main() {
    println!("Hello, world!");

    let s = "hello";

    println!("{}", s);

    let new_string = String::from("hello");

    println!("{}", new_string);

    let s2 = new_string;

    println!("{}", s2);

    let s3 = s2.clone();

    println!("{}, {}", s3, s2);

    let s4 = String::from("hello");

    takes_ownership(s4.clone());

    println!("{}", s4);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let s5 = gives_ownership();

    println!("{}", s5);

    let s6 = String::from("hello");

    let s7 = takes_and_gives_back(s6);

    println!("{}", s7);

    let s8 = String::from("hello");

    let (mut s9, len) = calculate_length(s8);

    println!("{} {}", s9, len);

    let len = calculate_length2(&s9);

    println!("{}", len);

    change(&mut s9);

    println!("{}", s9);

    let f1 = &mut s9;

    println!("{}", f1);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello from fn"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world from fn");
}
