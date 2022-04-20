// You could create a function for this solution as well, but be sure to try
// your program with varying lengths and types of arrays. Try one with all
// integers, another with negative numbers, and another with decimals.

fn main() {
    let array = [13, 5, 67, 43, 10, 80, 55];

    let result = sort_array(&array);

    println!("{:?}", result)
    // const length: usize = array.len();

    // sort_array::<length>(&mut array);

    // println!("{:?}", array);
}

// Only works on a fixed size array so far
fn sort_array(mut arr: &[i32; 7]) -> [i32; 7] {
    println!("{:?}", arr);
    let result = &arr.as_mut();
    
    *arr
}

// fn sort_array<const LEN: usize>(arr: &mut [i32]) -> &mut [i32; LEN] {
//     println!("{:?}", arr)
// }
