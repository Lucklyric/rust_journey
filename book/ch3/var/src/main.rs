fn main() {
    println!("Hello, world!");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("{}", tup.0);

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[0]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{}", months[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", a[0]);

    let a = [3; 5];

    println!("{},{}", a[0], a[1]);

    let mut first = a[0];

    println!("{}", first);

    first = first + 1;

    println!("{}", first);

    println!("{}", a[0]);

}
