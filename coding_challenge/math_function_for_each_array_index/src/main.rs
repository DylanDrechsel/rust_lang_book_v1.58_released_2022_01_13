// [1, 2, 4]
// [9, 5, 2]
// ---------------
// [10, 7, 6] --> Addition
// [-8, -3, 2] --> Subtraction
// [9, 10, 8] --> Multiplication
// [0.111, 0.4, 2] --> Division
// [1, 1, 0] --> Remainder


fn main() {
    let normal_array = vec![5, 12, 77, 54, 23];
    let longer_array = vec![1, 45, 33, 2, 23, 78];
    let shorter_array = vec![5, 3, 11, 31];

    let add_array_test_1 = add_array(&normal_array, &longer_array);
    println!("{:#?}", add_array_test_1)
}

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

    let result = (0..big_array.len()).map(|i| big_array[i] + small_array[i]).collect();

    // for (i, num) in big_array.iter().enumerate() {
    //     if small_array.contains(&small_array[i]) {
    //         println!("{}", small_array[i]);
    //         result.push(*num + small_array[i]);
    //         break;
    //     }
    //     result.push(*num);
    // }

    result

}