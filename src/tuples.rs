//Tuples group together values of different types
// Max 12 element 
pub fn run(){
    let person: (&str, &str, i32) = ("John", "Mars", 20);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}