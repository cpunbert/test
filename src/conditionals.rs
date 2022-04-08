pub fn keks(){

    let age: u8 = 18;
     let check_id:bool = false;
     let known = true;

    //if else
    if age>=18 && check_id || known { 
        println!("Pijemy");
    }else if age<18 && check_id{
        println!("Nie pijemy");
    }else{
        println!("dawaj dowodzik")
    }
    

    //short if check
    let is_old = if age >=18{true} else{false};
    println!("{}", is_old)

}
