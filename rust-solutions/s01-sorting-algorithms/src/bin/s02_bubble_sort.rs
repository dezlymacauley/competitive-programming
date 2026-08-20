/*
    ABOUT: Bubble Sort 
*/


// fn bubble_sort() {
//
// }

fn main() {
    let list_of_numbers: Vec<u8> = vec![40, 10, 60, 30, 50, 20];

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
}
