use std::io::ErrorKind;

use crate::libs::constants::MEMORY_MAX;

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
    COUNT,
}

#[derive(Debug)]
pub enum Opcodes {
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
    TRAP,   /* execute trap */
}

impl Opcodes {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Opcodes::BR),
            1 => Some(Opcodes::ADD),
            2 => Some(Opcodes::LD),
            3 => Some(Opcodes::ST),
            4 => Some(Opcodes::JSR),
            5 => Some(Opcodes::AND),
            6 => Some(Opcodes::LDR),
            7 => Some(Opcodes::STR),
            8 => Some(Opcodes::RTI),
            9 => Some(Opcodes::NOT),
            10 => Some(Opcodes::LDI),
            11 => Some(Opcodes::STI),
            12 => Some(Opcodes::JMP),
            13 => Some(Opcodes::RES),
            14 => Some(Opcodes::LEA),
            15 => Some(Opcodes::TRAP),
            _ => None,
        }
    }
}

pub enum ConditionalFlags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

pub trait RegisterStorageTrait {
    fn new() -> Self;
    fn load(&self, reg_location: Registers) -> Option<u16>;
    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind>;
}

#[derive(Debug)]
pub struct RegisterStorage {
    pub locations: [u16; Registers::COUNT as usize],
}

impl RegisterStorageTrait for RegisterStorage {
    fn new() -> Self {
        Self {
            locations: [0u16; Registers::COUNT as usize],
        }
    }

    fn load(&self, reg_location: Registers) -> Option<u16> {
        Some(self.locations[reg_location as usize])
    }

    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind> {
        self.locations[reg_location as usize] = instr;
        Ok(())
    }
}

pub trait MemomryTrait {
    fn new() -> Self;
    fn memory_read(&self, register_storage: &mut RegisterStorage) -> Option<u16>;
}

#[derive(Debug)]
pub struct Memory {
    pub locations: [u16; MEMORY_MAX],
}

impl MemomryTrait for Memory {
    fn new() -> Self {
        Self {
            locations: [0u16; MEMORY_MAX],
        }
    }

    fn memory_read(&self, register_storage: &mut RegisterStorage) -> Option<u16> {
        let memory_address_from_pc = register_storage.locations[Registers::PC as usize];
        let _ = register_storage.store(memory_address_from_pc + 1, Registers::PC);
        Some(memory_address_from_pc)
    }
}
