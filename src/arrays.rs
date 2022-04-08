use std::mem;


pub fn jd(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    //change value
    numbers[2] = 22222;

    println!("{:?}", numbers);

    //indexing

    println!("{}", numbers[2]);

    println!("lenth {}", numbers.len());

    //arrays are stack allocated
    println!("space in bytes {}", mem::size_of_val(&numbers));


    //slicing
    let slice: &[i32] =&numbers[2..3];
    println!("slice {:?}", slice);
}