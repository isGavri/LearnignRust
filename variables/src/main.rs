fn main() {
    //Mutability and inmutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    //Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of inner scope y is {y}");
    }
    println!("The value of outter scope y is {y}");
}
