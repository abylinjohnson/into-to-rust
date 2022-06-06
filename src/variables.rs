// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block scoped language 
pub fn run(){

    //Creating variables
    let name = "John";
    let mut age = 20;
    println!("My name is {} and my age is {}",
    name,age
    );
    age = 21;
    println!("My name is {} and my age is {}",
    name,age
    );

    //Define const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age ) = ("John", 37);
    println!("{} is {}", my_name, my_age);

}