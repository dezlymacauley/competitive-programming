fn bubble_sort(list: &mut Vec<u8>) {
    if list.len() < 2 {
        return;
    }

    let index_of_last_element: usize = list.len() - 1;

    loop {
        let mut at_least_one_swap_happened: bool = false;

        for index in 0..index_of_last_element {
            if list[index] > list[index + 1] {
                list.swap(index, index + 1);
                at_least_one_swap_happened = true;
            }
        }

        if !at_least_one_swap_happened {
            break;
        }
    }
}

fn main() {
    let mut list_of_numbers: Vec<u8> = vec![40, 10, 60, 30, 50, 20];
    bubble_sort(&mut list_of_numbers);
    println!("\nSorted List: {list_of_numbers:?}\n")
    //
    // Sorted List: [10, 20, 30, 40, 50, 60]
    //
}
