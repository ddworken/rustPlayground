use std::io; //for stdin

fn main(){
    let mut num = getIntFromStdin();
    let mut steps = 0;
    loop {
        if num <= 0 {
	    panic!("What do you want from me?!?!")
	}
        if num%2 == 0 {
            num = num / 2;
            steps += 1;
            println!("{0} is even, so I take half: {1}", num*2, num)
        }
	if num%2 != 0 && num != 1{
	    num = 3*num + 1; 
	    steps += 1;
	    println!("{0} is odd, so make 3n + 1: {1}", num, (num-1)/3);
	}
	if num == 1 {
	    println!("It took {} steps", steps);
	    break;
	}
    }
}

fn getIntFromStdin() -> u64 {
    println!("What number would you like to test? ");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()

    let val: u64 = val.trim() //trim() removes whitespace and new lines
    .parse() //parse gives us the number
    .ok().expect("Error: Please enter a positive number"); //once again, using ok() and expect() to deal with errors
    return val;
}
