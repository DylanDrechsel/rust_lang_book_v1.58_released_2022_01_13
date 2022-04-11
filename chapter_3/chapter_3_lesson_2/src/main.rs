fn main() {
    // unsigned int cannot be negative
    let guess_pos: u32 = "42".parse().expect("Not a number!");
    
    // signed int can be negative
    let guess_neg: i32 = "-42".parse().expect("Not a number!");

    // floating point ints
    let guess_float: f32 = 3.1;

    println!("Unsigned Int: {} || Signed Int: {} || Float Int: {}", guess_pos, guess_neg, guess_float);
}
