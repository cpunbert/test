pub fn run(){
    //Print to console 
    println!("Hello World");

    //Formating
    println!("{} is from {}","dis", "fordon");

    //Position arguments
    println!("{1} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} jebanie {activity}",name = "Disa", activity ="powieksza penisa");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o} ", 22, 22, 22);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "Hello "));

    //Basic math 
    println!("10/3 = {}", 10/3);
} 