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
    } else {
        print_help();
    }
}

fn parse_source(source: &str) -> String {
    let src: Vec<&str> = source.split_whitespace().collect();
    let src = src.join(" ").to_string();
    src
}

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

fn check_file(name: &str) -> bool {
    let parts: Vec<&str> = name.split('.').collect();
    if parts.last() == Some(&"poop") {
        return true;
    }
    false
}

fn print_help() {
    println!("poop-rs v0.1.0");
    println!("Enter a '.poop' file as argument to run it");
}