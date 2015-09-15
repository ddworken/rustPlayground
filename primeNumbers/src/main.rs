use std::io;    //import io from std to we can read a number from the user

fn main(){
    let size: usize = getSizeFromStdin(); //usize because this is the size of a vector 
   
    let mut vec = Vec::new();	//create a new vector
    for i in 0..size+1 {	//iterate through the whole thing and set every value to true
	vec.push(true)
    }
    for i in 2..((size as f64).sqrt()as usize)+1 { //as f64 converts to a f64 which has a .sqrt() method; we have to then convert back to usize so we can iterate up to that range; +1 is to make the standard < behavior function the same way as <= would in a java/c style for loop
        if vec[i] == true {
            let mut j = i; //must be mutable so we can increment it
            while i*j <= size { //using a while loop b/c for loops in rust don't have the ability to do i*j < x
                vec[i*j] = false; 
                j+= 1; //j++ is not a thing in rust
            }
        }
    } 
    let mut numPrimes = 0;
    for i in 2..size { 
        if vec[i] == true {
            println!("{}", i);
            numPrimes += 1;
        }
    }
    println!("In total we found {} prime numbers (printed above) from our sieve. ", numPrimes)
}

fn getSizeFromStdin() -> usize {
    println!("How big of a sieve would you like to make? ");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()

    let val: usize = val.trim() //trim() removes whitespace and new lines
    .parse() //parse gives us the number
    .ok().expect("Error: Please enter a number"); //once again, using ok() and expect() to deal with errors
    return val;
}
