pub fn run(){
    //print basic
    println!("hello");

    //print number
    println!("hello {}", 3);

    //print multiple values
    println!("hello {}, that's {}", 3, "nice");

    //print positional arguments
    //it places within the curly braces which order it should appear
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity="activity");

    //placeholder traits
    //it prints in binary, hex and octal
    println!("Binary:{:b} Hex:{:x} Octal:{:o}", 10, 10, 10);
    
    //placeholder for debug trait
    //tuple, it's basically used to store multiple values in a single varialbe
    println!("{:?}", (1, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10+10);
}
