pub fn run(){
    
    let mut hello = String::from("Hello ");


    //get len

    println!("Length: {}", hello.len());

    //push single char
    hello.push('W');

    //push string 
    hello.push_str("orld!");

    //capatcity bites

    println!("Capacity: {}", hello.capacity());

    //check if is empty
    println!("Capacity: {}", hello.is_empty());

    //containing 
    println!("Contains: {}", hello.contains("world"));


    //Replace
    println!("Replace: {}", hello.replace("World", "zydek"));

    //looping
    for word in hello.split_whitespace(){
        println!("{}", word);
    }


    //string with fixed capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion tsesting

    assert_eq!(1, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}