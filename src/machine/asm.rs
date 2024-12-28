use super::memory::{Addressable, LinearMemory};
use super::instructions::Instruction;
use super::registers;


pub struct Machine<'a> {
    memory: &'a dyn Addressable,
    registers: [u16; 12]
}

impl<'a> Machine<'a> {
    pub fn new(memory: &'a LinearMemory) -> Self {
        Self {
            memory,
            registers: [0; 12]
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = *self.registers.get(registers::Register::PC as usize).expect("Error while getting the PC Register");
        let instruction = self.memory.read(pc as usize).or(Err(""))?;
        let operator: u8 = (instruction >> 8) as u8;
        let _operand: u8 = (instruction & 0xFF) as u8;
        match Instruction::from(operator) {
            Instruction::Print => {
                println!("FOUND PRINT STATEMENT");
            }
            c => {
                println!("{:?}", c);
            }
        }
        self.registers[registers::Register::PC as usize] = pc + 1;
        Ok(())
    }
}
