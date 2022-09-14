fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // generic types

    let number_list = vec![34, 50, 25, 100, 65, 11111];

    let mut largest_n = &number_list[0];

    for number in &number_list {
        if number > largest_n {
            largest_n = number;
        }
    }

    println!("The largest number is {}", largest_n);

    let i32_list = vec![1, 2, 3, 3, 4, 576];

    let largest_n = largest_i32(&i32_list);

    println!("{}", largest_n);

    let char_list = vec!['a', 'b', 'c', 'd'];

    let largest_char = largest_char(&char_list);

    println!("{}", largest_char);

    let largest_v = largest(&char_list);

    println!("{}", largest_v);

    let both_integer = Point { x: 5, y: 10 };

    let both_float = Point { x: 5.0, y: 10.9 };

    let integar_and_flot = Point { x: 5, y: 10.0 };

    // print all points
    println!("{}, {}", both_integer.x, both_integer.y);
    println!("{}, {}", both_float.x, both_float.y);
    println!("{}, {}", integar_and_flot.x, integar_and_flot.y);

    let p1 = Point { x: 5, y: 10.4 };

    let p3 = p1.mixup(both_integer);

    println!("{}, {}", p3.x, p3.y);
}


