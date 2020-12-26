use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();  // init empty with type def
    println!("{:?}", v);

    let v1 = vec![1, 2, 3]; // init with macro
    println!("{:?}", v1);

    let mut v = Vec::new(); // update with push
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    // reading elements
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None        => println!("There is no third element."),
    }

    // iteraring over values
    for i in &v {
        println!("{}", i);
    }

    // mutate it
    for i in &mut v {
        *i += 50; // need to use derefernce op
    }
    
    for i in &v {
        println!("{}", i);
    }

    // enum to store different values
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //println!("{:?}", row);

    // strings

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    // concatenate

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // hashmaps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    // getting value out of it

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score); // Option enum

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update hashmap based on existing or not existing value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
