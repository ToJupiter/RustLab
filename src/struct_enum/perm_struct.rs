use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct complex_number {
    re: f64,
    im: f64
}

impl complex_number {
    pub fn complex_init(re: f64, im: f64) -> Self {
        return Self { re, im };
    }
    
    pub fn set_re_and_im(&mut self, re: f64, im: f64) {
        self.re = re;
        self.im = im;
    }

    pub fn distance_from_o(&self) -> f64 {
        return f64::sqrt(self.im * self.im + self.re * self.re);
    }

    pub fn return_longer_complex(self, other: complex_number) -> complex_number {
        let my_distance = self.distance_from_o();
        let other_distance = other.distance_from_o();

        match my_distance.total_cmp(&other_distance) {
            Ordering::Equal => return complex_number {..self},
            Ordering::Less => return complex_number {..other},
            Ordering::Greater => return complex_number {..self}
        }
    }

    fn set_to_max(&mut self, other: complex_number) {
        // Also fails if this does not derive Copy, Clone trait
        *self = self.return_longer_complex(other);
    }
}

pub fn borrow_and_set() {
    let mut two_three_i = complex_number::complex_init(3.5, 6.5);

    two_three_i.set_re_and_im(4.5, 7.5); // failed when immutable

    let two_three_i_borrow = &mut two_three_i;
    two_three_i_borrow.re += 1.0;
    two_three_i_borrow.im += 2.0;

    let mut three_four_i = complex_number::complex_init(3.0, 4.0);

    three_four_i.return_longer_complex(two_three_i);

    // this will be okay if Clone, Copy is enabled
    println!("The output is {}", two_three_i.distance_from_o());
    
}

pub fn ownership_manipulation() {
    let mut two_three_i = complex_number::complex_init(3.5, 6.5);
    let mut three_four_i = complex_number::complex_init(3.0, 4.0);

    two_three_i.set_to_max(three_four_i);
    
}