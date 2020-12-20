fn main() {
    let s = "hello"; // string literal on stack
    println!("{}", s);
    
    let mut s = String::from("hello"); // on heap can be mutated

    s.push_str(", world!");
    
    println!("{}", s);

    // move

    let x = 5;
    let y = x; // two separate values

    // move
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); // s1 no longer valid

    let s3 = s2.clone(); // both exists separately

    println!("s2 = {}, s3 = {}", s2, s3);

    let len = calculate_length(&s2);

    println!("{}", len);

    let a = String::from("hello, pal");
    let fw = first_word(&a);

    println!("{}", fw);

    let s = String::from("hello, world");

    let hello = &s[0..5];

    let world = &s[6..];  // slices = ref to parts of string in..notin

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
