pub fn run(){
    //default is i32
    let x = 1;
    println!("{}", x);
    
    
    //default is f64
    let y = 2.5;
    println!("{}", y);

    
    //add explicity type
    let z: i64 = 45454545454;
    println!("{}", z);

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);


    //boolean
    let is_active = true;


    //char
    let a1 = "a";


    println!("{:?}", (x, y, z, is_active));

    //get boolean from expression
    let is_greater: bool = 10 < 5; //in this example declaring the type (: bool) is optional
    println!("{}", is_greater);

}
