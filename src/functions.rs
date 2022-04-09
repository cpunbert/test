pub fn zyd(){
greeting("jebac", "hejterow");
let sum = add(69,2137);
println!("{}", sum);



let n3: i32 = 2137;
let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
println!("{}", add_nums(69,69));
}

pub fn greeting(activity: &str, name:&str){
    println!("{} {}", activity,name )
}

fn add(n1:i32, n2:i32) -> i32 {
n1 + n2
}