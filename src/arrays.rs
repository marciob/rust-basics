pub fn run(){
    let numbers: [i32; 5] = [1,2,3,4,5];
    
    println!("{:?}", numbers);

    //get single value
    println!("single value: {}", numbers[0]);

    let mut new_numbers: [i32; 5] = [1,2,3,4,5];

    //re-assign value
    new_numbers[2] = 20;

    println!("{:?}", new_numbers);

    //get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array ocupies {} bytes", std::mem::size_of_val(&numbers));
                                        //it could be imported at the top of file:
                                        //use std::mem;
                                        //them, in the code should be necessary just write mem::size_of_val();

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}
