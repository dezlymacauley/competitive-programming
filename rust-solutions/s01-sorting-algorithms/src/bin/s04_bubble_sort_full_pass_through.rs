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
    // println!("index_of_last_element: {index_of_last_element}");

    //_________________________________________________________________________

    // STEP: 5 => Create a loop that will repeatedly execute the algorithm
    // until a condition is met.

    loop {

        let mut at_least_one_swap_happened: bool = false;

        for index in 0..index_of_last_element {
            // NOTE: Always check that your loop is comparing
            // the correct indexes against each other.

            // println!("Comparing index {} to index {}", index, index + 1);
            // Comparing index 0 to index 1
            // Comparing index 1 to index 2
            // Comparing index 2 to index 3
            // Comparing index 3 to index 4
            // Comparing index 4 to index 5

            // STEP: 5 => Compare each element to the element next to it,
            // and swap their positions if needed.

            if list[index] > list[index + 1] {
                // You can use Rust's native `.swap()` method
                list.swap(index, index + 1);

                // Or you can use the manual snapshot method
                // let snapshot = list[index];
                // list[index] = list[index + 1];
                // list[index + 1] = snapshot;

                // record that at least one swap happend.
                at_least_one_swap_happened = true;
            }
        }

        // If no swaps happened then that means that the list is sorted,
        if !at_least_one_swap_happened {
            // break out of the loop and continue with the rest of the code
            // in the function.
            break;
        }

    }


    // STEP: 7 => Print out the sorted list

    println!("\nSorted List");
    println!(
        "{}\n",
        list.iter()
            .map(|element| element.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
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
