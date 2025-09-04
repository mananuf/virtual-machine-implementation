use std::io::ErrorKind;

pub enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, /* program counter */
    COND,
    COUNT
}

pub enum Opcodes
{
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP    /* execute trap */
}

pub enum ConditionalFlags{
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

pub trait  RegisterStorageTrait {
    fn new() -> Self;
    fn load(&self, reg_location: Registers) -> Option<u16>;
    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind>;
}

#[derive(Debug)]
pub struct RegisterStorage {
    pub locations: [u16; Registers::COUNT as usize]
}

impl RegisterStorageTrait for RegisterStorage {
    fn new() -> Self {
        Self { locations: [0u16; Registers::COUNT as usize] }
    }

    fn load(&self, reg_location: Registers) -> Option<u16> {
        Some(self.locations[reg_location as usize])
    }

    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind> {
        self.locations[reg_location as usize] = instr;
        Ok(())
    }
}