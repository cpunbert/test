use std::mem;


pub fn dis(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    //change value
    numbers[2] = 22222;

    println!("{:?}", numbers);

    //pushing
    numbers.push(44);

    println!("{:?}", numbers);


    //popping(remove last item_)

    numbers.pop();


    //indexing

    println!("{}", numbers[2]);

    println!("lenth {}", numbers.len());

    //vectors are stack allocated
    println!("space in bytes {}", mem::size_of_val(&numbers));


    //slicing
    let slice: &[i32] =&numbers[2..3];
    println!("slice {:?}", slice);

    //loops
    for x in numbers.iter() {
        println!("Number: {}",x)
    }


    //loop and mutates
    for x in numbers.iter_mut() {
        *x *= 2;
        println!("Loop mute {}", x)
    }

}