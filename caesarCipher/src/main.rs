use std::io; //for stdin

fn main(){
    let shiftAmount = getIntFromStdin();
    let decrypted = getStringFromStdin();
    let mut vec = decrypted.into_bytes();	//must be mut b/c we are modifying in place
    let len = vec.len();
    vec.remove(len-1); //so as to make it just a vector of the string to be encrypted
    for i in 0..len-1 {	//for char in the vector
        let mut char = vec[i];	
	if char <= 90 && char >= 65 {	//handling capital letters (numbers are constants from an ASCII table)
            char += shiftAmount;
            while char > 90 {	//while to handle shift amounts >26
                char -= 26;
            }
        }
	if char <= 122 && char >= 97 {	//handling lower case letters (numbers are constants from an ASCII table)
            char += shiftAmount;
            while char > 122 {
                char -= 26;
            }
        }
	vec[i] = char; 
    }
    println!("The encrypted string is: {}", String::from_utf8(vec).ok().expect("Unknown Error")); //not really sure what kind of error could cause this (I'm inclined to say it wouldn't happen given the above code) but can't hurt to have a bit of error handling
}

fn getIntFromStdin() -> u8 { 	//returns an unsigned 8 bit int
    println!("How many characters would you like to shift your encrypted string? ");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()

    let val: u8 = val.trim() //trim() removes whitespace and new lines
    .parse() //parse gives us the number
    .ok().expect("Error: Please enter a positive number"); //once again, using ok() and expect() to deal with errors
    return val;
}

fn getStringFromStdin() -> String {
    println!("What string would you like to encrypt?");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()
    return val;
}
