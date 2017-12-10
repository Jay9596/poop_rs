extern crate poop_rs;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let src = parse_source(&read_file(&args[1]));
        let r = poop_rs::run(&src).join(" ").to_string();
        println!("{}", r);
    } else if args.len() == 3 {
        if args[1] == "-o" || args[1] == "-out" {
            let o = poop_rs::poop_out(&args[2]);
            print!("Poop output:");
            println!("{}", o);
        } else {
            print_help();
        }
    } else {
        print_help();
    }
}

// parse_source converts the String read from file to a single spaced String to feed the run function.
// The read String has unnecessary whitesapces and linebreaks that are not needed.  
fn parse_source(source: &str) -> String {
    // Transform the single string to a vector of strings by splitting at whitespaces.
    let src: Vec<&str> = source.split_whitespace().collect();
    // Join the Vector to form a single String with even spaces.
    let src = src.join(" ").to_string();
    src
}

// read_file reads a file and returns the content as a String
fn read_file(name: &str) -> String {
    if check_file(name) {
        let path = Path::new(name);
        // println!("path {}", path.display());
        let mut file = match File::open(&path) {
            Err(why) => {
                println!("Error: {}", why);
                process::exit(1);
            }
            Ok(file) => file,
        };
        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Ok(_) => {}
            Err(why) => {
                println!("Error: {}", why);
                process::exit(1);
            }
        };
        text
    } else {
        panic!("Not a poop file");
    }
}

// check_file checks if the file given is a '.poop' file or not.
// Split the name at '.' and check if the last element is "poop"
fn check_file(name: &str) -> bool {
    let parts: Vec<&str> = name.split('.').collect();
    if parts.last() == Some(&"poop") {
        return true;
    }
    false
}

// print_help displays the help text for the program
fn print_help() {
    println!("poop_rs v1.0.0");
    println!("Enter a '.poop' file as argument to run it");
    println!("Enter a text to output poop code using the -o OR -out flag");
    println!("Eg: .\\poop_rs -out \"poopy rust\"");
}
