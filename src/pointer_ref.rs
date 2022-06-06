//Reference Pointer - point to a resource in memory
pub fn run(){
    //Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    //With non-primitive, if you assign another variable to a piece of data, the first
    // variable will np longer hold that value. you will need to use a refernce to point the resource

    //Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values : {:?}", (&vec1, vec2));
}