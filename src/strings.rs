// primitive str: immutable and fixed length
// string: growable, heap allocated data structure, used when you need to modify the data

pub fn run(){
    let msg = "Hello"; // primtive str

    println!("{}", msg);

    let mut hello = String::from("Hello "); // string

    //get length
    println!("Length: {}", hello.len());

    //push char
    hello.push('W'); // .push() is used to push chars, 'W' is a char, "W" is a string

    //push string
    hello.push_str("orld!"); // push_str() is used to push strings

    //capacity in bytes
    println!("capacity: {}", hello.capacity());

    //check if a string is empty
    println!("is empty: {}", hello.is_empty());

    //check if contains a word
    println!("contains word: {}", hello.contains("World"));

    //replace
    println!("replace: {}", hello.replace("World", "There"));

    //loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //assertion test
    //it compares if the first statement is equal to the second one
    assert_eq!(2, s.len()); // if it pass, it will show nothing, if it fail, it will show an error
    assert_eq!(11, s.capacity()); // if it pass, it will show nothing, if it fail, it will show an error

    println!("{}", hello);
}
