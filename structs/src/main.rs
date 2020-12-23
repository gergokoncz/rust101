// def
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn build_user(email: String, username: String) -> User { // field init syntax
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

    // init
    let user1 = build_user(String::from("some@email.com"), String::from("someuser"));
    let user2 = User {
        email: String::from("somother@email.com"),
        ..user1 // every other field based on user1
    };

    println!("{}", user2.username);

    // struct tuple def
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);

    // tuple implementation
    let rect1 = (50, 30);

    println!("The area of the rectange is {} square pixels ", tuple_area(rect1));

    // struct implementation
    let rect1 = Rectangle {
        length: 50,
        width: 30
    };

    println!("rect is {:?}", rect1);
    
    println!("The area of the rectange is {} square pixels ", struct_area(&rect1));
    println!("The area of the rectange is {} square pixels ", rect1.area());
    
    let rect2 = Rectangle {
        length: 40,
        width: 10
    };
    
    let rect3 = Rectangle {
        length: 45,
        width: 60
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // associated function for constructing instance
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size}
    }
}