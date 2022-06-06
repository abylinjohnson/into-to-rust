pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    //Re-assign value
    numbers[2] = 20;

    //Get single values
    println!("{}", numbers[0]);

    //Get array length
    println!("Array Length; {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
    
} 