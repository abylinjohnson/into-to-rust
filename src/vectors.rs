pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
   
    //Add on to vetor
    numbers.push(6);
    numbers.push(7);
    
    //Pop of the last value
    numbers.pop();

    println!("{:?}", numbers);

    //Re-assign value
    numbers[2] = 20;

    //Get single values
    println!("{}", numbers[0]);

    //Get array length
    println!("Vector Length; {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    //Get slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}",x);
    }
  
    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
} 