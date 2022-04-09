pub fn zyd(){

    //Primitive array
    let arr1 =[1,2,3];
    let arr2 = arr1;

    let  arr3 =vec![1,2,3];
    let  arr4 = &arr3;

    println!("Value: {:?}", (&arr3, arr4));

}