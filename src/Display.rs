use std::fmt;

struct WrapperArray {
    a: [i32; 2],
}

impl std::fmt::Display for WrapperArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {})", self.a[0])
    }
}

pub fn run() {
    let test = WrapperArray { a: [1, 2] };
    println!("Used Display: {}", test);
}
