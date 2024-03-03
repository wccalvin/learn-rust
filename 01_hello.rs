// This is a comment, ignored by the compiler
/* This is a block comment
 * To execute this program, first compile: rustc 01_hello.rs
 * rustc will produce a binary that can be executed
 */

/*
Note: The previous column of `*` for multi-line comment entirely for style.
There is no need for it.
 */

// This is the main function
fn main() {
    // Statements here are executed when the compiler binary is called
    println!("Hello World!"); // println! is a macro that prints text to console
    println!("I'm a Rustacean");

    let x = 5 + /*90 + */ 5;
    println!("value of x = {}", x);
}
