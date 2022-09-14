pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    //Get single value
    println!("Single Value: {}", numbers[1]);
    //Re-assign value
    numbers[2] = 20;
    println!("Re-assigned value: {}", numbers[2]);
    //Ger array length
    println!("Array Length: {}", numbers.len());
    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    let slice2: &[i32] = &numbers[0..3];
    println!("Slice2: {:?}", slice2);

    let test = String::from("sass");
    let chars: Vec<char> = test.chars().collect();

    println!("{}", test);

    for charElem in chars.iter() {
        println!("{}", charElem);
    }
}
