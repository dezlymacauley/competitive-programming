/*
    ABOUT: Bubble Sort - First pass through
*/

fn bubble_sort(list: &mut Vec<u8>) {
    // If the list has less 2 elements then end the function without doing
    // anything else.
    if list.len() < 2 {
        return;
    }

    // Get the index of the last element
    let index_of_last_element: usize = list.len() - 1;

    /*
        In Rust the `=` in `0..=index_of_last_element`
        means that it is an inclusive range.

        E.g. If index_of_last_element = 5
        Then the range is 0..6 (including 5)
    */
    for index in 0..index_of_last_element {
        if list[index] > list[index + 1] {
            let snapshot = list[index];
            list[index] = list[index + 1];
            list[index + 1] = snapshot;
        }
    }

    // Print out the sorted list
    println!("\nList afer first pass through:");
    println!(
        "{}",
        list.iter()
            .map(|element| element.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!();
    // List afer first pass through:
    // 10, 40, 30, 50, 20, 60
}

fn main() {
    // The indexes of each the element:     0   1   2   3   4   5
    let mut list_of_numbers: Vec<u8> = vec![40, 10, 60, 30, 50, 20];

    bubble_sort(&mut list_of_numbers);
}
