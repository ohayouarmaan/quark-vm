pub trait Addressable {
    fn read(&self, idx: usize) -> Result<u16, ()>;
    fn write(&mut self, idx: usize, value: u16) -> bool;
    fn copy(&mut self, src: usize, dst: usize) -> bool {
        if let Ok(read_value) = self.read(src) {
            if self.write(dst, read_value) {
                return true;
            } else {
                false;
            }
        } else {
            return false;
        }
        return false;
    }
}

pub struct LinearMemory {
    mem: [u16; 64 * 1024]
}

impl Addressable for LinearMemory {
    fn read(&self, idx: usize) -> Result<u16, ()> {
        if let Some(value) = self.mem.get(idx) {
            return Ok(*value);
        }
        Err(())
    }
    fn write(&mut self, idx: usize, value: u16) -> bool {
        if idx > 64*1024 {
            return false;
        }
        self.mem[idx] = value;
        return true;
    }
}

impl LinearMemory {
    pub fn new() -> Self {
        Self {
            mem: [0; 64*1024]
        }
    }
}
