// Primitive types
// Integers i32, u32, i64, u64 
// Floats f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

//Rust is staitically typed language, which means that it means
// that it must know the types of all the variables at compile time,
// however, the compiler can usually infer what type we want to usee based on
// the value and how we use it.
pub fn run(){
    //Default is "i32"
    let _x = 1;

    //Default is "i64"
    let _y = 2.5;

    //Add explicit type
    let _j: i64 = 545454545454;

    let a1 = 'a';
    println!("{:?}", (a1));

    //Find the max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);
}