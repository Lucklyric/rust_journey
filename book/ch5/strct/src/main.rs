struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // print all fields of the struct
    println!(
        "email {}, username {}, active {}, sign_in_count {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let user2 = User {
        email: user1.email.clone(),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!(
        "email {}, username {}, active {}, sign_in_count {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    println!("{}", user1.email);

    let user3 = User {..user2};

    println!(
        "email {}, username {}, active {}, sign_in_count {}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    );

}
