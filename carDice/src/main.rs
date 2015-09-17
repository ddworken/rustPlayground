use std::io; //for stdin
extern crate rand;  //rand is a crate, not part of std
use rand::{thread_rng, Rng};    //the parts from rand we need

fn main(){
    let numberOfDice = 6;	//use to change the number of dice
    let insuranceCost = 1200;   //use to change the cost of the insurance
    let carCost = 800;		//use to change the cost of renting a car for the day
    let costOfPlaying = 2;	//use to change the cost of playing the game

    let numRolls = getNumRollsFromStdin();	//how many rolls they want to play
    let mut revenue = 0;			//money taken in by the fair
    let mut costs = carCost + insuranceCost;	//total costs
    let mut carsWon = 0;			//total cars won by the player

    for i in 0..numRolls {			
        revenue += costOfPlaying; 		//increment revenue every time they play
        let mut num6 = 0;			//used to count how many 6s they got
        for j in 0..numberOfDice {
            let mut rng = thread_rng();		
            let roll = rng.gen_range(1, 7);	//between 1 and 7 b/c 7 isn't included (so 1 and 6)
            if roll == 6 {			//if they got a 6, then increment the above counter
                num6 += 1;
            }
        }
        if num6 == numberOfDice { 		//if they got as many 6s as they rolled, then winner! 
            carsWon += 1; 
        }
    }
    println!("You spent {0} dollars and won {1} cars. This means each car cost you {3} dollars. The fair had a profit of {2} dollars. ", revenue, carsWon, revenue - costs, revenue / carsWon);  //status line at the end of the fair
}

fn getNumRollsFromStdin() -> u64 { //returns unsigned (positive) 64 bit int (so they can play MANY times :D)
    println!("How many times would you like to run the simulation? ");
    let mut val = String::new(); //mut means we can change the value of it; String::new() is a new blank String (Strings are growable)
    io::stdin() //returns an instance of Stdin
    .read_line(&mut val) //gives read_line a reference to val; it must be mutable so it can modify it and store the line from stdin into the string
    .ok().expect("Error could not read from stdin"); //check if everything is ok, if it isn't we have a problem so use expect() which will panic!()

    let val: u64 = val.trim() //trim() removes whitespace and new lines
    .parse() //parse gives us the number
    .ok().expect("Error: Please enter a number"); //once again, using ok() and expect() to deal with errors
    return val;
}
