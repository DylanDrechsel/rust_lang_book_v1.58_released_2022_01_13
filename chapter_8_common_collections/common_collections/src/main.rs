// Array size are known at compile time
// Vector size not known at compile time

use unicode_segmentation::UnicodeSegmentation; // Used for iterating over Grapheme Strings
use std::collections::HashMap; // Brings the HashMap type into scope

fn main() {
    // Allocated to the stack --> Values cannot change
    let array = [1, 2, 3];

    // Allocated to the heap --> Values can change
    let mut vector: Vec<i32> = Vec::new();

    // Pushing values onto "vector"
    vector.push(1);
    vector.push(2);
    vector.push(3);

    // Puts contents in their own scope
    {
        // Allows declartion of values on creation
        let vector2 = vec![1, 2, 3];
    }
    // "vector2" leaves scope and is not long able to be accessed

    // Accessing the values within a vector
    {
        let mut vector = vec![1, 2, 3, 4, 5];

        let third = &vector[2];
        
        // Wont work because it will change the memorylocation that variable "third" is pointing at
        // vector.push(6);
        println!("The third element is {}", third);

        // Get Method --> Error Handling
        match vector.get(20) {
            // Variable exist case
            Some(third) => println!("The third element is {}", third),

            // Variable does not exist case
            None => println!("There is no third element")
        }
    }

    // Iterating over all the indexs of a Vector
    {
        let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

        for value in &mut vector {
            *value += 50;
        }

        for value in &vector {
            println!("{}", value)
        }
    }

    // Enums & Vectors
    {
        // enum can only have one of their "variants" fulfilled
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    
        // Returns --> 3
        match &row[0] {
            SpreadsheetCell::Int(i) => println!("{}", i),
            _ => println!("Not an integer!")
        }
    
        // Returns --> Not an integer!
        match &row[1] {
            SpreadsheetCell::Int(i) => println!("{}", i),
            _ => println!("Not an integer!")
        }
    }

    // Strings
    {
        // Strings are stored as a collection of UTF-8 encoded bytes
        // Since Strings are UTF-8 encoded were able to write them in many different languages (non-computer)
        {
            let s1 = String::new();
            let s2 = "initial contents";
            let s3 = s2.to_string();
            let s4 = String::from("initial contents");

            // Just like a Vector Strings can grow and srink in size
            let mut s = String::from("foo");
            s.push_str("bar");
            s.push('!');
            // Returns --> "foo bar!"
        }

        // Concat string together
        {
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");
            let s3 = s1 + &s2;

            //println!("{}", str1) --> Will throw error because the value of "str1" moved to "str3"
            //println!("{}", str2) --> Will return the value of "str2" because the value was borrowed on "str3"
            println!("{}", s3);
        }

        // Format (format!) Strings together --> Format DOES NOT take ownership of the variables
        {
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");
            let s3 = format!("{}{}", s1, s2);

            println!("{}", s3);
        }

        // Indexing into a String
        {
            let hello = String::from("नमस्ते"); // Indian for "Hello"
            // let c = hello[0]; --> Will not work because indexs cannot be refrenced by an integer

            // The 3 ways a word is represented in Unicode

            // Bytes
            // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

            println!("--------------- ");
            for bytes in "नमस्ते".bytes() {
                println!("{}", bytes)
            }

            // Scalar values
            // ['न', 'म',' स', ' ्',  'त', ' ्']

            println!("--------------- ");
            for chars in "नमस्ते".chars() {
                println!("{}", chars)
            }

            // Grapheme clusters
            // ["न", "म", "स", "त"]

            println!("--------------- ");
            for graph_values in "नमस्ते".graphemes(true) {
                println!("{}", graph_values)
            }

            println!("--------------- ");
        }
    }

    // Hash Maps --> Imported the HashMap Type above the "main" function
    {
       {
        // Hash Maps allow you to store key/value pairs
        let blue = String::from("Blue");
        let yellow = String::from("Yellow");

        // Creates HashMap
        let mut scores: HashMap<String, i32> = HashMap::new();

        // Inserts values to HashMap
        // Moves variable ownership to the HashMap
        scores.insert(blue, 10);
        scores.insert(yellow, 50);

        // Get value from HashMap
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        println!("{:?}", score);

        for (key, value) in &scores {
            println!("{}: {}", key, value)
        }
       }

       // Updating HashMap
       {

       }
    }
}
