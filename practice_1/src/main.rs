use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

/* Generate a random number from 1 to 20 */

    let random_number:u32 = rand::thread_rng().gen_range(1..=20);
    
loop {
/* Get user input */

   println!("Enter a number: "); 

   let mut user_input = String::new();
    
   io::stdin()
    .read_line(&mut user_input)
    .expect("Didn't work well");

   /* convert the string entered to a u32 number */ 

   let user_input: u32 = match user_input.trim().parse().unwrap(){
        Ok(num)=>num,
        Err(_)=>break
    };

   println!("You entered: {}", user_input);

/* Compare user input with the generated number */

   match user_input.cmp(&random_number){
    Ordering::Less => println!("You gussed low. Try Again."),
    Ordering::Greater => println!("You gussed High. Try Again."),
    Ordering::Equal => {
        println!("Good Job. You Win !!!"); 
        break;}
        }
    }
}