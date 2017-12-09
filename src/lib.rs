const CHARACTERS: &str = "0123456789abcdefghijklmnopqrstuvwxyz.,-!?+*<>#@$€§%&/()[]";

enum Instructions {
    MoveRight,
    MoveLeft,
    Output,
    OutputCaps,
    PrintStr,
    EmptyStr,
}

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

pub fn run(source: &str) -> Vec<String> {
    let characters: Vec<char> = CHARACTERS.chars().collect();
    let instructions = parse(source);

    let size = characters.len();
    let mut ins_pointer = 0;
    let mut mem_pointer = 0;
    let mut output = String::new();
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

#[cfg(test)]
mod test {
    use super::run;

   #[test]
   fn jay() {
       assert_eq!(run("eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat POOP puke puke puke puke puke puke puke puke puke poop eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat eat poop sniff")
            .join(" ").to_string(),
       "Jay");
   }

   #[test]
   fn check_9596() {
       assert_eq!(run("eat eat eat eat eat eat eat eat eat poop puke puke puke puke poop eat eat eat eat poop puke puke puke poop sniff")
        .join(" ").to_string(),
       "9596");
   }
}