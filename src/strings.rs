// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure 
//- Use when you need to modify or own string data
// Parenthesis are not required when creating.

pub fn run() {
    //Allocate to heap with string::from and make it mutable
    // &str can't edit, essentially same as "&'static str"
    let mut hello = String::from("hello "); //make it heap allocated. 

    //get length
    //Allocate to 
    println!("length: {}", hello.len());
    println!("{}", hello);

    //Push char
    hello.push('W');

    println!("{}", hello);

    //Push String
    hello.push_str("orld!");

    println!("{}", hello);

    //ex
    //let mut hello2: &str = ("hello2");
    //hello2.push('W'); 
    //This will give you a error as &str can't be editted as it is only pointing the reference of its where string is stored.

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty: Produce bool value "true" if it's empty
    println!("Is Empty: {}", hello.is_empty());

    //Contains: Case sensitive
    println!("Contains World: {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    //split at where space is located
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    //nth iterates through the string and return the Option<T> 
    //Option<T> is a value that 
    //As iterator may or may not have next value, .nth will return the Option<T>
    //Option<T> consists either Some() (there is a value) or none (nothing in the T)
    //To print the Option<T> value, you must use Debug traits !dbg() or {:?}
    println!("{:?}", s.chars().nth(1));
    dbg!(s.chars().nth(1));

    //If you would like to use Display traits and show actual UTF-8 value,
    //you must use .unwrap()

    println!("{}", s.chars().next().unwrap());
    println!("Test {:?}", s.chars().nth(3)); //Produce none
    //println!("Test2 {}", s.chars().nth(3).unwrap()); unwrapping non-existent index will cause panic

    //Assertion testing
    let mut b: bool = true;
    assert!(b); //It doesn't do anything which means no error ;)
    b = false;
    //assert!(b); If false, this assertion will panic.

    //If both variables are equal then it's fine otherwise it gives error.
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}