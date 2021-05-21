use std::io; // std library and io module

// main function; entry point of the program
fn main() {

    //println! is a macro
    println!("Enter your weight(kg) :");
    
    // varible declaration to take input of user
    let mut input = String::new();
    
    // read a string from standard input, &mut -> because, all the variables are immutable in Rust to use it further we make it reference,
    // unwrap() -> if error occurs while taking input it terminates.
    io::stdin().read_line(&mut input).unwrap();

    // trim -> remove whitespaces,
    // parse -> convert to datatype needed.
    let weight = input.trim().parse().unwrap();
    // function call 
    let mars_weight = calculate_weight_on_mars(weight);
	println!("Weight on mars {}",mars_weight);

}

// function def to convert weight
// -> f32 : return type of the function

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.711    // function return

}
