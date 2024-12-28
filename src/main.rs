mod machine;
use machine::memory::Addressable;

fn main() -> Result<(), &'static str> {
    let mut memory = machine::memory::LinearMemory::new();
    memory.write(0, 1);
    let mut vm = machine::asm::Machine::new(&mut memory);
    vm.step()
}
