extern crate ncurses;

mod brainfuck;

use ncurses::*; // watch for globs

fn main() {
    initscr();
    println!("Hello, world!");
    let input = "<>+-.,[]";
    brainfuck::compile(input.as_bytes());
}
