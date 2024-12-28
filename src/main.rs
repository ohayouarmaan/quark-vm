mod machine;
use machine::memory::Addressable;

fn main() -> Result<(), &'static str> {
    let mut memory = machine::memory::LinearMemory::new();
    memory.write(0, 0b00000001_00000000);
    let mut vm = machine::asm::Machine::new(&mut memory);
    vm.step()
}
