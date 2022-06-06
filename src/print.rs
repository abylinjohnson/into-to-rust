pub fn run() {
    println!("Hello From Rust!");
    println!("{},{}",4, 2);

    //Positional Arguments
    println!("{0} is from {1}", "John","Mars");

    //Named Arguments
    println!("{name} likes to play {activity}",
    name="John", activity="Basket Ball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",
    10,10,10,
    );

    //Placeholders for debuging
    println!("{:?}", (12, true, "hello")); 

    //Basic Maths
    println!("10 + 10 = {}",10+10);
}