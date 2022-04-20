// You could create a function for this solution as well, but be sure to try
// your program with varying lengths and types of arrays. Try one with all
// integers, another with negative numbers, and another with decimals.

fn main() {
    let array = [13, 5, 67, 43, 10, 80, 55];
    let array2 = [1, 163, 35, 647, 43, 150, 820, 5534, 78, 45, 254];

    let result = sort_array(&array);
    let result2 = sort_array(&array2);

    println!("{:?}", result);
    println!("{:?}", result2);
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![0; arr.len()];

    for (i, number) in arr.iter().enumerate() {
        vector[i] = *number;
    }

    vector.sort();

    vector
}
