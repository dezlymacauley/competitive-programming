/*
    ABOUT: Swapping elements using a snapshot variable

    This is useful for swapping two elements because you can take a snapshot
    of an elements value before performing the swap.
*/

fn main() {
    // The indexes of each the element:     0   1   2   3
    let mut list_of_numbers: Vec<u8> = vec![20, 10, 40, 30];

    //_________________________________________________________________________

    // Swapping 20 and 10

    // Take a snapshot of the element at index 0
    let mut snapshot = list_of_numbers[0];

    // Copy index 1 to index 0
    list_of_numbers[0] = list_of_numbers[1];

    // Replace index 1 with the snapshot.
    list_of_numbers[1] = snapshot;

    //_________________________________________________________________________

    // Swapping 40 and 30

    // Take a snapshot of the element at index 2
    snapshot = list_of_numbers[2];

    // Copy index 3 to index 2
    list_of_numbers[2] = list_of_numbers[3];

    // Replace index 3 with the snapshot.
    list_of_numbers[3] = snapshot;

    //_________________________________________________________________________

    println!("\nSorted List:");
    println!(
        "{}",
        list_of_numbers
            .iter()
            .map(|element| element.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!();

    // Sorted List:
    // 10, 20, 30, 40
}
