mod main_machine;

use main_machine::Machine;
use quark_vm::shared::memory::{LinearMemory, Addressable};
use quark_vm::shared::registers;

fn main() -> Result<(), &'static str> {
    let mut memory = LinearMemory::new();
    memory.write(0x0, 0b00000001_00000000); // print
    memory.write(0x1, 0b00000010_00000001); // push 1
    memory.write(0x2, 0b00000001_00000000); // print
    memory.write(0x3, 0b00000010_00000010); // push 
    memory.write(0x4, 0b00000001_00000000); // print
    memory.write(0x5, 0b00000100_00000000); // add_stack
    memory.write(0x6, 0b00000001_00000000); // print
    memory.write(0x7, 0b00000110_00000000); // Halt
    let mut vm = Machine::new(&mut memory);
    while vm.running {
        let _ = vm.step();
    }
    Ok(())
}
