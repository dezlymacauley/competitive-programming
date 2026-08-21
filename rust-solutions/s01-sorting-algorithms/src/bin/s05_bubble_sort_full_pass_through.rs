/*
    ABOUT: Bubble Sort - Full Pass Through
*/

// STEP: 1 => Define the parameters and return type of the function

fn bubble_sort(list: &mut Vec<u8>) {
    //_________________________________________________________________________

    // STEP: 2 => Ensure that the function only attempt to sort lists that
    // have at least two elements

    if list.len() < 2 {
        return;
    }

    //_________________________________________________________________________

    // STEP: 3 => Get the index of the last element

    let index_of_last_element: usize = list.len() - 1;
    println!("index_of_last_element: {index_of_last_element}");

    //_________________________________________________________________________

    // STEP: 4 => Create a loop that will compare each index 
    // to the next index in the list

    // Please note that the range is exclusive
    // E.g. If index_of_last_element is 5
    // Then 0..5 means, index 0 to index 5 (excluding index 5)

    // This is correct, because you want index 5 to be excluded.
    // If index 5 was included you would get an "out of bounds" error,
    // because he for loop would eventually try to compare index 5,
    // to index 6 (which woud make the program crash because index 6
    // does not exist)

    for index in 0..index_of_last_element {

        // NOTE: Always check that your loop is comparing the correct indexes

        println!("Comparing index {} to index {}", index, index + 1);
        // Comparing index 0 to index 1
        // Comparing index 1 to index 2
        // Comparing index 2 to index 3
        // Comparing index 3 to index 4
        // Comparing index 4 to index 5
    }

    // loop {
    //
    //     let mut swap_happend: bool = false;
    //
    //     for index in 0..index_of_last_element {
    //         if list[index] > list[index + 1] {
    //             let snapshot = list[index];
    //             list[index] = list[index + 1];
    //             list[index + 1] = snapshot;
    //
    //             swap_happend = true;
    //         }
    //     }
    //
    //     if swap_happend == false {
    //         break;
    //     }
    // }

    // Print out the sorted list
    // println!("\nSorted List");
    // println!(
    //     "{}",
    //     list.iter()
    //         .map(|element| element.to_string())
    //         .collect::<Vec<String>>()
    //         .join(", ")
    // );
    // println!();
    // Sorted List:
    // 10, 20, 30, 40, 50, 60
}

fn main() {
    // Create the list
    // The indexes of each the element:     0   1   2   3   4   5
    let mut list_of_numbers: Vec<u8> = vec![40, 10, 60, 30, 50, 20];

    // Sort the list
    bubble_sort(&mut list_of_numbers);
}
