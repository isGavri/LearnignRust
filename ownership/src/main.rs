// Enables Rust to make memory safety guarantees without needing a garbage collector
// Set of rules that govern how Rust manages memory, generally programing languages use a garbage
// collector that looks for no longer used memory, and in another languages the programer must
// explicitly allocate and free memory.
// In Rust its used a different aproach, the memory safety is managed trough a system of ownership
//  with a set of rules such that if any of those rules are violated the program wont compile
//
//  --- The stack and the heap ---
//  Rust is a system programing language so wether a value is on the heap or the stack affects how
//  the languages behaves.
//  Both are parts of your memory aviable for your program at runtime but their structure its
//  different.
//  * The stack stores the values in the order it gets them and removes them in the
//  opposite order. Last in, first out. Its like a pile. All of the values stored in the stack must
//  have a known and fixed sized. Data with unkown size at compile time or size that might change
//  should be stored in the heap. Adding elements to the stack is called pushing and removing them
//  its called poping.
//  * The heap its less organized, when you place data in the heap you request a certain amount of
//  memory. The memory allocator finds an empty spot in the heap that its big enough marks it as
//  being used and return a pointer which is the address of that location. Because the pointer to
//  the heap is known and of fixed sized, you can store it on the stack, but when you want the data
//  you must follow the pointer.
//
//  ** Pushing to the stack is faster than pushing to the heap and accessing data in the heap is
//  also slower that accessing from the stack.
//
//  --- Ownership rules
//  * Each value in Rust has an owner
//  * There can only be one owner at a time
//  * When the owner goes out of scope the value will be dropped
fn main() {
    let string = "Hello"; // the variable remains valid until it goes out of scope
    println!("{string}, world!");
    let mut s = String::from("Hello"); // As opposed to string literal (the example below) string
                                       // from the standar library are mutable
    s.push_str(", world!");
    println!("{s}");
    // But the real difference between these two type of strings is how they deal with memory
}
