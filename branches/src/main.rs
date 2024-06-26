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
    // Because if its an expresion (has return value) we can use it in a let statement;
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
        counter += 1; //return whats after the break
    }
    //We can bind the result of a loop to a variable by adding a expesion after the break statement
    let count_stop = loop {
        counter += 1;
        if counter >= 5 {
            break counter * counter;
        }
    };
    println!("The counter  stopped at {count_stop}");

    multiple_loops();
    conditional_loops_with_while();
    while_and_for_loops();
}
fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break affect the inner loop
            }
            if count == 2 {
                break 'counting_up; // break affects the outter loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
fn conditional_loops_with_while(){
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number-=1;
    }
    println!("Lift of");
}
fn while_and_for_loops(){
    let a  = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("The value is {}", a[i]);
        i+=1;
    }
    for element in a {
        println!("The value is {element}");
    }
    for number in (1..5).rev() { // The first element in the range is not included 
        println!("The value is {number}");
    }
}
