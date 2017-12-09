# poop_rs :poop:
:poop: [Poop](https://esolangs.org/wiki/Poop) Interpretor  
This is a simple poop intrepreter written in Rust. 

## [Poop](https://esolangs.org/wiki/Poop)   
Poop is an esoteric programming language.  
-----
[esolangs Wiki](https://esolangs.org/wiki/Poop)

It has 6 basic instructions.
| Command | Description |
| ------ | ------ |
| eat | Move pointer to right |
| puke | Move pointer to left |
| poop | Add pointed character to string |
| POOP | Add pointed characted to string uppercased |
| sniff | Show string |
| flush | Empty string and set pointer to 0 |

Supported charactes in Poop 1.0  
0123456789abcdefghijklmnopqrstuvwxyz.,-!?+*<>#@$€§%&/()[]

## Test 
There are tests provided in the 'lib' file to test the functionality of the interpreter library.  
To run the tests `cargo test`

## Example
Examples for poop files are provided in examples directory.  
To run the examples `cargo run -- .\example\hello.poop`  

## Build
To build the binary for your platform use the cargo build tools.  
`cargo build` for development builds. Faster builds for testing the binaries.  
`cargo build --release` for release build. This takes time to build due to the optimizations performed by the compiler.  

## Usage
poop_rs can run the '.poop' files and even generate a poop source for a given text

1. Interpret poop file  
`poop_rs.exe hello.poop`  
The interpretor will run the code in the poop file and print the output to standard output.

2. Generate a poop source code  
`poop_rs.exe -out "Rust"`  
The '-out' Or '-o' flag is used to specify the code from text.  