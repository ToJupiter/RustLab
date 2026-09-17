/*
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

 */
pub fn vector_iterator() {
    let v1: Vec<i32> = vec![10,20,30,40];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&20));
    assert_eq!(v1_iter.next(), Some(&30));
    assert_eq!(v1_iter.next(), Some(&40));
    assert_eq!(v1_iter.next(), None);

    /*
     * sum() is one of the method that calls next()
     */
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // FnMut: &mut self
    // FnOnce: self
    // Fn: &self
    
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}


