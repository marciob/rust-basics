pub fn run(){
    let name = "Brad";
    let mut age = 37;
    
    println!("My name is {}", name);
    
    println!("My age is {}", age);

    age = 38;

    println!("My age is {}", age);

    const ID: i32 = 0001;

    println!("ID: {}", ID);

    //assigning multiple variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} {}", my_name, my_age);

}
