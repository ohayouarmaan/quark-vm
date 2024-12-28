pub enum Register {
    r0 = 0, r1, r2, r3, r4, r5, r6, r7, // GENERAL PURPOSE REGISTERS
    pc, sp, fp, fl // Program Counter, Stack Pointer, Frame Pointer, Flag Register
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Register::r0,
            1 => Register::r1,
            2 => Register::r2,
            3 => Register::r3,
            4 => Register::r4,
            5 => Register::r5,
            6 => Register::r6,
            7 => Register::r7,
            8 => Register::pc,
            9 => Register::sp,
            10 => Register::fp,
            11 => Register::fl,
            _ => panic!("Invalid register requested")
        }
    }
}
