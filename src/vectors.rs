pub fn run(){

        let mut numbers: Vec<i32> = vec![1,2,3,4,5];
        
        println!("{:?}", numbers);
    
        //get single value
        println!("single value: {}", numbers[0]);
    
        let mut new_numbers: [i32; 5] = [1,2,3,4,5];

        //add on to vector
        numbers.push(5);
        numbers.push(6);

        //pop off last valeu
        numbers.pop();
    
        //re-assign value
        new_numbers[2] = 20;

        println!("{:?}", numbers);
    
        println!("{:?}", new_numbers);
    
        //get vector length
        println!("Vector length: {}", numbers.len());
    
        //Arrays are stack allocated
        println!("Vector ocupies {} bytes", std::mem::size_of_val(&numbers));
                                            //it could be imported at the top of file:
                                            //use std::mem;
                                            //them, in the code should be necessary just write mem::size_of_val();
    
        //get slice
        let slice: &[i32] = &numbers[0..2];
        println!("Slice: {:?}", slice);

        //loop through vector values
        for x in numbers.iter(){
            println!("Number: {}", x);
        }


        //loop and mutate values
        for x in numbers.iter_mut() {
            *x *=2;
        }

        println!("Numbers Vec: {:?}", numbers);
}
