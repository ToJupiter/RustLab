pub fn vector_largest() {
    let number_list = vec![100, 38, 20, 99];

    let mut largest_num = &number_list[0];
    for number in &number_list {
        if number > largest_num {
            largest_num = number;
        }
    }
    println!("The largest number is: {:?}", largest_num);

    // The correct way to do an abstraction
    let new_number_list = vec![1000, 29, 38, 20];
    let new_largest = find_largest(&new_number_list);
    println!("The new way to find the largest value returns {:?}", new_largest);
}

/* This is an abstraction of the operation to find the largest element in a vector */
pub fn find_largest(list: &[i32]) -> &i32 {
    let mut largest_num = &list[0];

    for item in list {
        if item > largest_num {
            largest_num = item;
        }
    }
    return largest_num;
}


