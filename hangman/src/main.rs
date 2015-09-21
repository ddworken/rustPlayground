use std::io; //for stdin
use std::path::Path; //for filenames 
use std::fs::File; //for the file
use std::io::Read; //to read from the above file
extern crate rand;  //rand is a crate, not part of std
use rand::{thread_rng, Rng};    //the parts from rand we need

fn main(){ 
    let maxGuesses = 7;     //configure this to change the max number of guesses allowed
    let mut guesses = Vec::new();   //a vector of chars
    let mut incorrectGuesses = 0; 
    let mut randomWord = getRandomWord();
    loop { //equivalent to while True: 
        let guess = getPlayersGuess();  
        println!("You guessed: {}", guess);

        //Begin backdoor code
        if guess == 124 as char {   //simple semi-obfuscated backdoor (124 is the ASCII code for "|")
            println!("The word is: {}", randomWord);
        }
        //End backdoor code

        else { //else b/c is they activated the backdoor we shouldn't count it against them
            guesses.push(guess);    //add the guess to the guesses array
            guesses.sort();         //sort it
            guesses.dedup();        //then dedup it (we are nice so we don't count repeated guesses against them)
            if !isCorrect(randomWord.clone(), guess){
                incorrectGuesses += 1; 
            }
        }

        let bytes = randomWord.clone().into_bytes();    //.clone() because it isn't copyable; .into_bytes() to get the u8/char
        let mut numCorrect = 0; //the number of characters they have gotten correct so far
        for byte in bytes { //for each character in the random word
            let mut done = false; //used to track if the character was guessed
            for char in guesses.clone() { //.clone() because it isn't copyable
                let guessByte = char as u8; // convert char to u8
                if guessByte == byte {  //if they are equal
                    print!("{} ", byte as char);
                    done = true;
                    numCorrect += 1;
                }
            }
            if !done { //if they didn't get it, then print _ as the blank space
                print!("_ ");
            }
        }
        print!("So far you have guessed: "); //printing the guesses they have made so far
        for char in guesses.clone() {
            print!("{} ", char);
        }
        println!(". ");
        if numCorrect == randomWord.len() { //handling if they won
            println!("Congratulations you won in {0} guesses! The word was {1}. ", incorrectGuesses, randomWord); 
            break; 
        }
        else { //just letting them know how many guesses they used
            println!("So far you have used {} guesses. ", incorrectGuesses);
        }
        if incorrectGuesses >= maxGuesses { //handling if they lost
            println!("You lose! Game over. The word was: {}", randomWord);
            break;
        }
    }
}

fn isCorrect(randomWord: String, guess: char) -> bool { //used to check if a guess is correct so we know if we should increment the incorrect counter
    for char in randomWord.chars(){
        if char == guess {
            return true;
        }
    }
    return false;
}

fn getRandomWord() -> String{ //returns a randomly chosen word from wordlist.txt
    let wordlist = getFile("wordlist.txt"); //wordlist is a Vec<string>
    let mut rng = thread_rng();
    let num: u32 = rng.gen_range(0, (wordlist.len() - 1) as u32);
    return wordlist[num as usize].clone();  //as usize because it is an index and .clone() because non-copyable
}

fn getFile(filename: &str) -> Vec<String> { //largely taken from stackexchange b/c the docs on opening files aren't great. So yes this was taken from online but I can explain each line
    let mut file = match File::open(filename) { //this is creating the file variable
        Ok(file) => file,			//standard ok() and Err() syntax to check for errors
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();	//blank string to hold what we get from the file
    file.read_to_string(&mut file_contents)	//take the previous file and store the contents in the above string
        .ok()					//standard ok().expect() syntax
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")	//create the lines vector from splitting on which is a new line
        .map(|s: &str| s.to_string())			//map the str to a String b/c we want a vector of Strings not strs
        .collect();					//used with map to stick it all into the vector
    return lines;
}


fn getPlayersGuess() -> char{ //lets them guess a character
    loop { //loop is equivalent to while True; we need it b/c then it keeps on asking them for one until it gets a valid guess
        println!("Guess a character!");
        let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
        io::stdin() //returns an instance of Stdin
        .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
        .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()
        if val.len() == 2 {	//2 b/c it includes the new line
            return val.into_bytes()[0] as char; 
        }
    }
}
