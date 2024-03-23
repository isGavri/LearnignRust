fn main() {
    println!("Hello, world!");
    another_function();
    number_function(120);
    print_labeled_measurements(20, 'm');
    number_function(number());
    number_function(plus_one(number()));
}

//just a function that doesnt return anything but executes some code
fn another_function() {
    println!("Another function");
}

//Function that takes parameters/arguments and uses them in the statement
fn number_function(x: u8) {
    println!("The number is {x}");
}

//Function that takes more that one parameter
fn print_labeled_measurements(x: u8, y: char) {
    println!("The measurements is {x}{y}");
}

//Functions that return something
fn number() -> u8 {
    20
}

//Function that takes parameters and return something
fn plus_one(x: u8) -> u8 {
    x + 5
}
