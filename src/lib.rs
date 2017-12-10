// constant string with the valid characters of Poop v1.0
const CHARACTERS: &str = "0123456789abcdefghijklmnopqrstuvwxyz.,-!?+*<>#@$€§%&/()[]";

// All valid instructions of Poop v1.0
enum Instructions {
    MoveRight,  // eat
    MoveLeft,   // puke
    Output,     // poop
    OutputCaps, // POOP
    PrintStr,   // sniff
    EmptyStr,   // flush
}

// parse takes the source code as String and returns a vector of Instructions
fn parse(source: &str) -> Vec<Instructions> {
    let ops : Vec<&str> = source.split(' ').collect();
    let mut instructions = vec![];

    for i in 0..ops.len() {
        match ops[i] {
            "eat" => instructions.push(Instructions::MoveRight),
            "puke" => instructions.push(Instructions::MoveLeft),
            "poop" => instructions.push(Instructions::Output),
            "POOP" => instructions.push(Instructions::OutputCaps),
            "sniff" => instructions.push(Instructions::PrintStr),
            "flush" => instructions.push(Instructions::EmptyStr),
            _ => {},       
        }
    }

    instructions
}

// run takes in the poop source code and returns a vector of Strings it generates.
pub fn run(source: &str) -> Vec<String> {
    let characters: Vec<char> = CHARACTERS.chars().collect();
    // A vector of instructions to perform
    let instructions = parse(source);

    // The size of the character vector of the valid characters in the language.
    let size = characters.len();
    // Instruction pointer
    let mut ins_pointer = 0;
    // Memory pointer
    let mut mem_pointer = 0;
    // String to fill during the run
    let mut output = String::new();
    // The vector to return at the end
    // At each succesful sniff, the "output" String is pushed and cleared.
    let mut output_text = vec![];

    while let Some(instruction) = instructions.get(ins_pointer) {
        match *instruction {
            Instructions::MoveRight => if mem_pointer + 1 == size {
                mem_pointer = 0;
            }else {
                mem_pointer += 1;
            },
            Instructions::MoveLeft => if mem_pointer == 0 {
                mem_pointer = size - 1;
            }else {
                mem_pointer -= 1;
            },
            Instructions::Output => output.push(characters[mem_pointer]),
            Instructions::OutputCaps => {
                if let Some(c) = characters[mem_pointer].to_uppercase().next() {
                    output.push(c)
                }
            },
            Instructions::PrintStr => {
                output_text.push(output.clone());
                output.clear();
            },
            Instructions::EmptyStr => {
                output.clear();
                mem_pointer = 0;
            }
        }

        ins_pointer += 1;
    }

    output_text
}

// poop_out takes in a text as String and returns poop code as String
pub fn poop_out(text: &str) -> String {
    // String to return at the end.
    let mut output = String::new();

    // Memory pointer to the valid characters of the language
    let mut mem_pointer = 0;
    // The input text is transformed into a vector of characters.
    let text: Vec<char> = text.chars().collect();
    for i in text {
        match i {
            ' ' => {    // Space is denoted by a `sniff` and a `flush`
                //  println!("Space");
                output.push_str("sniff\nflush\n");
                mem_pointer = 0;
            }
            _ => {
                if let Some(index) = CHARACTERS.find(i.to_lowercase().next().unwrap()) {
                    // Condition to check if instruction shoud be `eat` or `puke`
                    // Whether move the pointer right or left.
                    if index > mem_pointer {
                        let diff = index - mem_pointer;
                        mem_pointer += diff;
                        // Append `eat` untile the pointer reaches the required position.
                        for _ in 0..diff {
                            output.push_str("eat ");
                        }
                    } else {
                        let diff = mem_pointer - index;
                        mem_pointer -= diff;
                        // Append `puke` until the pointer reaches the required position. 
                        for _ in 0..diff {
                            output.push_str("puke ");
                        }
                    }
                }
                // Check for uppercased character
                if i.is_uppercase() {
                    output.push_str("\nPOOP\n");
                } else {
                    output.push_str("\npoop\n");
                }
            }
        }
    }
    // Append `sniff` at the end of the code
    // There is no output without this at the end.
    output.push_str("\nsniff");
    output
}

// Tests for the library functions: 'run' and 'poop_out'
#[cfg(test)]
mod test {
    use super::{run, poop_out};

    // Check the 'run' function for the string "jay"
   #[test]
   fn jay() {
       assert_eq!(run("eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat POOP puke puke puke puke puke puke puke puke puke poop eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat poop sniff")
            .join(" ").to_string(),
       "Jay");
   }

    // Check the 'run' function for the string "9596"
   #[test]
   fn check_9596() {
       assert_eq!(run("eat eat eat eat eat eat eat eat eat poop puke puke puke puke poop eat eat eat eat poop puke puke puke poop sniff")
        .join(" ").to_string(),
       "9596");
   }

    // Chech the 'poop_out' function for the string "9596"
   #[test]
   fn out_9596() {
       assert_eq!(poop_out("9596"), 
       "eat eat eat eat eat eat eat eat eat \npoop\npuke puke puke puke \npoop\neat eat eat eat \npoop\npuke puke puke \npoop\n\nsniff\n");
   }

    // Check the 'poop_out' function for the string "Aa" 
   #[test]
   fn out_cap_a() {
       assert_eq!(poop_out("Aa"),
       "eat eat eat eat eat eat eat eat eat eat \nPOOP\n\npoop\n\nsniff\n");
   }
}