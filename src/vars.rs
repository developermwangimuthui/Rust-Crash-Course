// Variables hold primitive data or reference to data
// Variables are immutable by default 
// Rust is a block scoped language 
pub fn run (){
    let name = "David";

    // To make variables muttable we use the mut keyword 
    let mut age = 24;
    println!("My name is {} and I am {}",name,age);
    age=25;
    println!("My name is {} and I am {}",name,age);

    // Define constants (Variables that dont change)
    // You must provide the type for the variable eg i32 means it is 32 bit 
    //Usually the variable name is all uppercase ie ID
    const ID :i32 = 001;
    println!("ID :{}",ID);

    // Assign multiple variables 
    let(my_name ,myhobby) =("David","coding");

    println!("{} likes {}",my_name,myhobby);



}