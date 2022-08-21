/************************************************************************\
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
This is the basic hello-world syntax for the RUST programming language.
execute the following commands in unixterminal to compile and run:

% rustc main.rs
% ./main
---> Hello, world!

*Notes: Rust style indents with four spaces, not TAB.
        Lines of code end in semicolon



~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
\************************************************************************/

fn main () {

    println!("Hello, world!");
}







/************************************************************************\
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
BREAKDOWN:

fn main() {      --> Defines a function named 'main'. Main is special: it
}                    is always the first code that runs in every executable
*                    rust program. Main has no parameters and returns
*                    nothing. All function bodies wrapped in '{}'.

println!("Hello, world!")
--> This is the basic syntax to print a text string to the console. It
    calls something called a 'rust macro'. Function calls do not use a '!'.
    Macros are different from normal functions, and sometimes follow
    different rules


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
\************************************************************************/
