pub fn vector_largest() {
    let number_list = vec![100, 38, 20, 99];

    let mut largest_num = &number_list[0];
    for number in &number_list {
        if number > largest_num {
            largest_num = number;
        }
    }
    println!("The largest number is: {:?}", largest_num);
}

pub fn find_largest(list: &[i32]) -> &i32 {
    let mut largest_num = &list[0];

    for item in list {
        if item > largest_num {
            largest_num = item;
        }
    }
    return largest_num;
}

