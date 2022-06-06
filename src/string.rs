//Primitive String = Immutable fixed length string somewhere in memory 
// String = growable, heap -allocated data structure Use when you need to modify or own

pub fn run(){
    //let hello = "Hello from rust"; //Primitive
    let mut hello = String::from("Hello"); //Mutable String 

    //Get length
    println!("length: {}", hello.len());

    //Push char and string
    hello.push(',');
    hello.push_str(" World");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is Empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'world' {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through a string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(3, s.len());

    println!("{}", s);

    //println!("{}", hello);

    

}
