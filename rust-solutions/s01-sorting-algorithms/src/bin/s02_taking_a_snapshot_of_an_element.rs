/*
    ABOUT: Taking a snapshot of an element
*/

fn main() {

    // The indexes of each the element:     0   1   2   3
    let mut list_of_numbers: Vec<u8> = vec![87, 10, 40, 30];

    // This variable `snapshot` is a snapshot of the element at index 0.
    // The `snapshot` variable gets its own copy of the element at index 0.

    // If the element at index 0 changes after this line,
    // the `snapshot` variable will not be affected.
    let snapshot = list_of_numbers[0];

    println!("list_of_numbers[0] is: {}", list_of_numbers[0]);
    // list_of_numbers[0] is currently 87

    println!("snapshot before index 0 changes: {snapshot}");
    // snapshot before index 0 changes: 87

    list_of_numbers[0] = 20; 
    println!("list_of_numbers[0] is now {}", list_of_numbers[0]);
    // list_of_numbers[0] is now 20
    
    println!("snapshot after index 0 changes: {snapshot}");
    // snapshot after index 0 changes: 87

    println!("{list_of_numbers:?}");
    // [20, 10, 40, 30]
}
