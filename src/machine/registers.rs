pub enum Register {
    R0 = 0, R1, R2, R3, R4, R5, R6, R7, // GENERAL PURPOSE REGISTERS
    PC, SP, FP, FL // PROGRAM COUNTER, STACK POINTER, FRAME POINTER, FLAG REGISTEr
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            9 => Register::SP,
            10 => Register::FP,
            11 => Register::FL,
            _ => panic!("Invalid register requested")
        }
    }
}
