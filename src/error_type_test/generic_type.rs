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

/*
 * PartialOrd limits the trait of the generic type T
 */
pub fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item == largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T
}

pub fn sample_point() {
    let integer_impl = Point {x: 5, y: 10};
    let float_impl = Point {x: 1.0, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    fn distance_to_core(&self) -> f32 {
        return (self.x.powi(2) + self.x.powi(2)).sqrt();
    }
}
