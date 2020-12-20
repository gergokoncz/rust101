fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("{}", x);
    another_function();
    
    let x = five();
    println!("{}", x);

}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}