fn main() {
    println!("Hello, world!");

    let s = String::from("hello 1234");

    println!("{}", s);

    let len = first_word(&s);

    println!("{}", len);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
