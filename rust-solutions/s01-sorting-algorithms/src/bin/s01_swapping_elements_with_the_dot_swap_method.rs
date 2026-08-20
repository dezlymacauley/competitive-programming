/*
    ABOUT: Swapping elements with the .swap() method
*/

fn main() {

    // The indexes of each the element:     0   1   2   3
    let mut list_of_numbers: Vec<u8> = vec![20, 10, 40, 30];

    // Swapping 20 and 10
    list_of_numbers.swap(0, 1);
    
    // Swapping 40 and 30
    list_of_numbers.swap(2, 3);

    // Printing the elements of the list, separated by a comma.
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
