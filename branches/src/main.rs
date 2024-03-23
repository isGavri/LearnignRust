fn main() {
    let number = 0;

    //Using if statements

    if number > 10 {
        println!("The number is greater than 10");
    } else if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is between 5 and 10");
    }

    //Using the if inside a declaration/let statement

    let condition = true;
    let num = if condition { 10 } else { 5 };
    println!("The condition returned {num}");

    // *****  Loops *****

    // loop creates an infinite loop until you explicitly tell it to stop
    let mut counter = 0;
    loop {
        println!("One more {} left", 10 - counter);
        // If it meets the condition the code inside containing the break will execute and end the
        // loop
        if counter >= 10 {
            counter = 0;
            break;
        }
        counter += 1;
    }
    //We can bind the result of a loop to a variable by adding a expesion after the break statement
    let count_stop = loop {
        counter += 1;
        if counter >= 5 {
            break counter * counter;
        }
    };
    println!("The counter  stopped at {count_stop}");
}
