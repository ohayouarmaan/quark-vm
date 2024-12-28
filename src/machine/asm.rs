use super::memory::{Addressable, LinearMemory};
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
        let pc = *self.registers.get(registers::Register::pc as usize).expect("Error while getting the PC Register");
        let instruction = self.memory.read(pc as usize).or(Err(""))?;
        let operator: u8 = (instruction >> 8) as u8;
        match operator {
            0b1 => {
                println!("FOUND THE FIRST INSTRUCTION");
            }
            c => {
                println!("{}", c);
            }
        }
        Ok(())
    }
}
