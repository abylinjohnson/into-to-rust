use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "John";
    //println!("Args: {}", command);
    if command == "Hello"{
        println!("Hello {}", name);
    } else{
        println!("Who are you");
    }
}