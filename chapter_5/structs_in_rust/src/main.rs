struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Houses the functions and methods for Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("drechseldylan@gmail.com"),
        username: String::from("BubbaBlount"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("ChangedUsername");

    let user2 = build_user(String::from("User2@gmail.com"), String::from("User2UserName"));

    let user3 = User {
        email: String::from("User3@gmail.com"),
        username: String::from("User3UserName"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels.", rect.area());
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1
    };
}
