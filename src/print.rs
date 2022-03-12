pub fn run (){
    println!("Print from main file");

    //Basic formarting
    println!("Number is {}",1);
    println!("{} likes {}","David","chicken");
    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}","David","Nakuru","code");

    // Named Arguments
    println!("{name} likes to play {game}", name="David",game="rugby");

    // Place holder traits 
    // Prints the Binary , Hexadecimal and Octal values for the  parameters given 
    println!("Binary: {:b} Hex {:x} Octal {:o}",10,10,10);

    // Placeholder for debug trait
    println!("{:?}",(12,true,"Hello"));

    // Basic Math
    println!("10 + 10 = {}",10+10);

}