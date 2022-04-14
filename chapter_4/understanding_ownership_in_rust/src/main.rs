fn main() {
    // ----- Ownership Rules -----
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped./
    
    {   // s is not valid here, it's not yet declared
        let s: &str = "hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s) Will throw error because value was moved

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let new_str = give_ownership();
    println!("new_str = {}", new_str);

    let s3 = String::from("hello");
    let s4 = takes_and_gives_back(s3);
    println!("s4 = {}", s4);

    let s5 = String::from("hello");
    let len = calculate_length(&s5);
    println!("The length of {} is {}", s5, len);

    let mut s6 = String::from("hello world");
    let hello = &s6[..5];
    let world = &s6[6..];

    let word = first_word(&s6);

    println!("{}", word);
    s6.clear()

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn give_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}
