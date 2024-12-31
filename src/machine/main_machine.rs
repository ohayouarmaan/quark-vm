use quark_vm::shared::memory::{Addressable, LinearMemory};
use quark_vm::shared::instruction::Instruction;
use quark_vm::shared::registers;

pub struct Machine<'a> {
    memory: &'a mut dyn Addressable,
    registers: [u16; 12],
    pub running: bool
}

impl<'a> Machine<'a> {
    pub fn new(memory: &'a mut LinearMemory) -> Self {
        let vm = Self {
            memory,
            registers: [0; 12],
            running: true
        };
        return vm;
    }

    fn push_to_stack(&mut self, value: u16) -> Result<(), &'static str> {
        let mut sp = self.registers[registers::Register::SP as usize];
        if sp == 0 {
            self.memory.write(0xFFF, value as u16);
            self.registers[registers::Register::SP as usize] = 0xFFF;
            Ok(())
        } else {
            sp -= 1;
            self.memory.write(sp as usize, value as u16);
            self.registers[registers::Register::SP as usize] = sp;
            Ok(())
        } 
    }

    fn pop_stack(&mut self) -> Result<u16, ()> {
        let prev_value = self.memory.read(self.registers[registers::Register::SP as usize] as usize);
        self.registers[registers::Register::SP as usize] += 1;
        return prev_value;
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = *self.registers.get(registers::Register::PC as usize).expect("Error while getting the PC Register");
        let instruction = self.memory.read(pc as usize).or(Err(""))?;
        let operator: u8 = (instruction >> 8) as u8;
        let operand: u8 = (instruction & 0xFF) as u8;
        match Instruction::from(operator) {
            Instruction::NOP => {
                // No operation
                self.registers[registers::Register::PC as usize] = pc + 1;
            }
            Instruction::Print => {
                // Should print the top of the stack
                println!("{:?}", self.memory.read(self.registers[registers::Register::SP as usize] as usize)
                    .expect("Error while reading from stack"));
            }
            Instruction::PushStack => {
                // Should push to the stack
                let _ = self.push_to_stack(operand as u16);
            }
            Instruction::PopStack => {
                // Should pop from the stack
                let _ = self.pop_stack();
            }
            Instruction::AddStack => {
                let x = self.pop_stack().expect("error while popping stack");
                let y = self.pop_stack().expect("error while popping stack");
                let _ = self.push_to_stack(x + y);
            }
            Instruction::Halt => {
                self.running = false;
            }
            _ => {
                panic!("INSTRUCTION NOT FOUND");
            }
        }
        self.registers[registers::Register::PC as usize] = pc + 1;
        Ok(())
    }
}
