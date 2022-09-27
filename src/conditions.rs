pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;
    
    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: sorry, you have to leae");
    } else {
        println!("Bartender: I need to see your ID");
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};   // it reads in that way:
                                                        // if age is higher or equal than 21, sets to true
                                                        // otherwise, sets to false
    println!("Is Of Age: {}", is_of_age);
}
