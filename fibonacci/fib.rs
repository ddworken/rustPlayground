use std::io;	//import io from std

fn main(){
    println!("What value would you like to calculate the fib on? ");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()
    
    let val: i32 = val.trim() //trim() removes whitespace and new lines
    .parse() //parse gives us the number
    .ok().expect("Error: Please enter a number"); //once again, using ok() and expect() to deal with errors

    let fibValue: i32 = val;
    println!("fib({0}) is {1}. ", fibValue, fib(fibValue));
}

fn fib(x: i32) -> i32{
    if x < 2 {
        return x;
    }
    return fib(x-1) + fib(x-2);
}
