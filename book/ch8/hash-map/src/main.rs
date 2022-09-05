use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");

    let socore = scores.get(&team_name).unwrap_or(&0);

    println!("socore: {}", socore);

    let non_exist_team_name = String::from("Red");

    let non_exist_socore = scores.get(&non_exist_team_name).unwrap_or(&0);

    println!("non_exist_socore: {}", non_exist_socore);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue2"), 25);

    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let mut filed_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(&field_name, &filed_value);

    println!("field_name: {}", field_name);

    // delete map all 
    map.clear();        

    println!("{:?}", map);

    println!("{}", filed_value);
}
