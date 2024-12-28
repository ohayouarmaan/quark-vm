#[derive(Debug)]
pub enum Instruction {
    NOP,
    Print,
    PushStack,
    PopStack,
    AddStack,
    LoadRegister,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 =>   Self::NOP,
            1 =>   Self::Print,
            2 =>   Self::PushStack,
            3 =>   Self::PopStack,
            4 =>   Self::AddStack,
            5 =>   Self::LoadRegister,
            _ => panic!("Invalid instruction")
        }
    }
}
