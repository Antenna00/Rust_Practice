use std::io;

pub fn run() {

    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    
    println!("入力値: {}", x);
    
}