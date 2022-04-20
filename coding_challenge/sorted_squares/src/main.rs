fn main() {
    let arr = vec![-7, -3, 2, 3, 11];
    let result: Vec<i32> = sorted_squares(&arr);

    println!("Result: {:?}", result);
}

fn sorted_squares(arr: &[i32]) -> Vec<i32> {
    // Initalizes Vector of arr length filled with 0's
    let mut result: Vec<i32> = vec![0; arr.len()];
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut i: usize = arr.len() - 1;

    while start <= end {
        println!("i: {}", i);
        println!("arrStart: {}", arr[start]);
        println!("arrEnd: {}", arr[end]);
        if arr[start].abs() > arr[end].abs() {
            println!("hit if");
            result[i] = arr[start] * arr[start];
            start += 1;
            if i == 0 { break; }
            i -= 1
        } else {
            println!("hit else");
            result[i] = arr[end] * arr[end];
            end -= 1;
            if i == 0 { break; }
            i -= 1;
        }
    }
    result
}
