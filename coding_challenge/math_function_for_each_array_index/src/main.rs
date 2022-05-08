// [1, 2, 4]
// [9, 5, 2]
// ---------------
// [10, 7, 6] --> Addition (Complete)
// [-8, -3, 2] --> Subtraction
// [9, 10, 8] --> Multiplication
// [0.111, 0.4, 2] --> Division
// [1, 1, 0] --> Remainder


fn main() {
    let normal_array = vec![5, 12, 77, 54, 23];
    let longer_array = vec![1, 45, 33, 2, 23, 78];
    let shorter_array = vec![5, 3, 11, 31];

    let add_array_test_1 = add_array(&normal_array, &longer_array);
    let add_array_test_2 = add_array(&normal_array, &shorter_array);
    let add_array_test_3 = add_array(&shorter_array, &longer_array);
    println!("{:?}", add_array_test_1);
    println!("{:?}", add_array_test_2);
    println!("{:?}", add_array_test_3);

    let sub_array_test_1 = sub_array(&normal_array, &longer_array);
    let sub_array_test_2 = sub_array(&normal_array, &shorter_array);
    let sub_array_test_3 = sub_array(&shorter_array, &longer_array);
    println!("{:?}", sub_array_test_1);
    println!("{:?}", sub_array_test_2);
    println!("{:?}", sub_array_test_3);

    let mult_array_test_1 = mult_array(&normal_array, &longer_array);
    let mult_array_test_2 = mult_array(&normal_array, &shorter_array);
    let mult_array_test_3 = mult_array(&shorter_array, &longer_array);
    println!("{:?}", mult_array_test_1);
    println!("{:?}", mult_array_test_2);
    println!("{:?}", mult_array_test_3);
}

// fn find_array_size(a1: &Vec<i64>, a2: &Vec<i64>) -> i64 {

// }

fn add_array(a1: &Vec<i64>, a2: &Vec<i64>) -> Vec<i64> {
    let big_array = 
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        };

    let small_array =
        if big_array == a1 {
            a2
        } else {
            a1
        };

    let mut big_array = big_array.iter();
    let mut small_array = small_array.iter();

    let mut result = Vec::new();

    loop {
        match (big_array.next(), small_array.next()) {
            (Some(x), Some(y)) => {
                let sum = x + y;
                result.push(sum);
            },
            (Some(x), None) => result.push(*x),  
            (None, Some(y)) => break, 
            (None, None) => break,
        }
    }

    result
}

fn sub_array(a1: &Vec<i64>, a2: &Vec<i64>) -> Vec<i64> {
    let big_array = 
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        };

    let small_array =
        if big_array == a1 {
            a2
        } else {
            a1
        };

    let mut big_array = big_array.iter();
    let mut small_array = small_array.iter();

    let mut result = Vec::new();

    loop {
        match (big_array.next(), small_array.next()) {
            (Some(x), Some(y)) => {
                let sum = x - y;
                result.push(sum);
            },
            (Some(x), None) => result.push(*x),  
            (None, Some(y)) => break, 
            (None, None) => break,
        }
    }

    result
}

fn mult_array(a1: &Vec<i64>, a2: &Vec<i64>) -> Vec<i64> {
    let big_array = 
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        };

    let small_array =
        if big_array == a1 {
            a2
        } else {
            a1
        };

    let mut big_array = big_array.iter();
    let mut small_array = small_array.iter();

    let mut result = Vec::new();

    loop {
        match (big_array.next(), small_array.next()) {
            (Some(x), Some(y)) => {
                let sum = x * y;
                result.push(sum);
            },
            (Some(x), None) => result.push(*x),  
            (None, Some(y)) => break, 
            (None, None) => break,
        }
    }

    result
}

fn division_array(a1: &Vec<i64>, a2: &Vec<i64>) -> Vec<i64> {
    let big_array = 
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        };

    let small_array =
        if big_array == a1 {
            a2
        } else {
            a1
        };

    let mut big_array = big_array.iter();
    let mut small_array = small_array.iter();

    let mut result = Vec::new();

    loop {
        match (big_array.next(), small_array.next()) {
            (Some(x), Some(y)) => {
                let sum = x / y;
                result.push(sum);
            },
            (Some(x), None) => result.push(*x),  
            (None, Some(y)) => break, 
            (None, None) => break,
        }
    }

    result
}

fn divisable_by_two_array(a1: &Vec<i64>, a2: &Vec<i64>) -> Vec<i64> {
    let big_array = 
        if a1.len() > a2.len() {
            a1
        } else {
            a2
        };

    let small_array =
        if big_array == a1 {
            a2
        } else {
            a1
        };

    let mut big_array = big_array.iter();
    let mut small_array = small_array.iter();

    let mut result = Vec::new();

    loop {
        match (big_array.next(), small_array.next()) {
            (Some(x), Some(y)) => {
                let sum = (x + y) % 2;
                result.push(sum);
            },
            (Some(x), None) => result.push(0),  
            (None, Some(y)) => break, 
            (None, None) => break,
        }
    }

    result
}