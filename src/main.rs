extern crate clap;

use clap::{Arg, App};

fn main() {
    println!("Hello, world!");
    let matches = App::new("Lox Interpreter")
        .version("0.0.0")
        .author("Zoe McCarthy <zoemccarthy12@gmail.com>")
        .about("An interpreter for the Lox Programming Language based off of the book Crafting Interpreters by Bob Nystrom <craftinginterpreters.com>")
        .arg(Arg::with_name("input_file")
            .help("Sets an input file to interpret")
            .index(1))
        .get_matches();
    if let Some(input_file) = matches.value_of("input_file") {
        println!("Interpreting input file: {}", input_file);
    }
    else {
        println!("Not using any input file, going into interactive mode.");
    }
    println!("Goodbye, world!");
}
