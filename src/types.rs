pub fn run(){

    let x = 1;//default "i32"

    let y = 3.55;//default "f64"

    let z: i64 = 34234234234234234;


    //Find max size

    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);


    let is_active: bool = true;

    let a1 = 'a';

    let face = '\u{1F98D}';

    let is_greater = z > 2137;

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));



}