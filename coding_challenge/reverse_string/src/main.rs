fn main() {
    let str = String::from("hello world!");
    let characters = str.chars();
    let mut result_string = String::new(); 
    // let mut result_vector = Vec::new(); --> Used for return as a Vector

    for letter in characters.rev() {
        // result_vector.push(letter); --> Pushing the "letters" to the Vector

        result_string.push_str(&letter.to_string())
    }

    // println!("{:#?}", result_vector); --> Printing the vector in a pretty format
    println!("{}", result_string)
}
