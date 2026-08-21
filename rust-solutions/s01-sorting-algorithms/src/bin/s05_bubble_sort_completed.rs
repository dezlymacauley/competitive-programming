/*
    ABOUT: Bubble Sort - Completed
*/

fn bubble_sort(list: &mut Vec<u8>) {
    // If the list has less 2 elements then end the function without doing
    // anything else.
    if list.len() < 2 {
        return;
    }

    let index_of_last_element: usize = list.len() - 1;

    loop {

        let mut swap_happend: bool = false;

        for index in 0..index_of_last_element {
            if list[index] > list[index + 1] {
                let snapshot = list[index];
                list[index] = list[index + 1];
                list[index + 1] = snapshot;

                swap_happend = true;
            }
        }

        if swap_happend == false {
            break;
        }
    }

    // Print out the sorted list
    println!("\nSorted List");
    println!(
        "{}",
        list.iter()
            .map(|element| element.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!();
    // Sorted List:
    // 10, 20, 30, 40, 50, 60
}

fn main() {
    // The indexes of each the element:     0   1   2   3   4   5
    let mut list_of_numbers: Vec<u8> = vec![40, 10, 60, 30, 50, 20];

    bubble_sort(&mut list_of_numbers);
}
