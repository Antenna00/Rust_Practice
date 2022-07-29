extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");
    
    //Basic Formatting
    println!("{}", 1); //You must have a string literal when printing the number.
    println!("{} is from {}", "Brad", "Mass");//Placeholder is required for Rust when printing at any circumstance.

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    //Named Arguments
    println!("{name} likes to play {activity}",
    name = "John",
    activity = "Baseball"
    );

    //Placeholder traits 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 ,10 ,10);

    //Placeholder for debug trait "Tuple"
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);

    let p = "Goldenさんしゅき";
    let o = p.as_bytes();
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(o, width, &mut writer).unwrap();

}
