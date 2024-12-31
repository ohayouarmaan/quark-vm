use quark_vm::shared::instruction::Instruction;
use std::fs;

pub struct Reader<'a> {
    file_path: &'a str,
    file_content: String
}

#[derive(Debug)]
pub struct AssemblyInstruction {
    instruction_type: Instruction,
    arguments: Vec<u16>
}

impl<'a> Reader<'a> {
    pub fn new(file_path: &'a str) -> Result<Self, &'static str> {
        let file_content = fs::read_to_string(file_path).expect("Error while reading the file");
        
        Ok(Self {
            file_path,
            file_content
        })
    }

    fn parse_number(&self, string_value: &'a str) -> Result<u16, ()> {
        let mut radix = 10;
        let string_value_without_leader: String;
        match string_value.chars().nth(0) {
            Some(value) => {
                if value == '$' {
                    radix = 16;
                    string_value_without_leader = string_value[1..].to_string();
                } else if value == '#' {
                    radix = 2;
                    string_value_without_leader = string_value[1..].to_string();
                } else {
                    string_value_without_leader = string_value.to_string();
                }
            }
            None => {

                return Err(())
            }
        }
        Ok(u16::from_str_radix(&string_value_without_leader, radix).unwrap())
    }

    pub fn parse(&self) -> Vec<AssemblyInstruction> {
        let content = self.file_content
            .split("\n")
            .filter(|x| *x != "")
            .map(|x| x.split(" ")
                .filter(|y| *y != "")
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut instructions: Vec<AssemblyInstruction> = vec![]; 
        for ins in content {
            match *ins.get(0).unwrap() {
                "push" => {
                    let args = ins.iter()
                        .skip(1)
                        .map(|x| self.parse_number(x)
                            .expect("Error while parsing the argument to number"))
                        .collect::<Vec<_>>();
                    instructions.push(AssemblyInstruction {
                        instruction_type: Instruction::PushStack,
                        arguments: args
                    });
                }
                _ => {
                    unimplemented!("Instruction not implemented yet.");
                }
            }
        }
        return instructions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_parsing() {
        
    }
}
