use thiserror::Error;
use tracing::info;

use crate::libs::constants::MEMORY_MAX;

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Invalid register: {0}")]
    InvalidRegister(u16),
}

#[derive(Debug)]
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
    fn load(&self, reg_location: u16) -> Result<u16, RegisterError>;
    fn store(&mut self, instr: u16, reg_location: u16) -> Result<(), RegisterError>;
    fn update_flags(&mut self, destination_register: u16) -> Result<(), RegisterError>;
}

#[derive(Debug)]
pub struct RegisterStorage {
    pub locations: [u16; Registers::COUNT as usize],
}

impl RegisterStorageTrait for RegisterStorage {
    fn new() -> Self {
        info!("initializing new register storage");

        Self {
            locations: [0u16; Registers::COUNT as usize],
        }
    }

    fn load(&self, reg_location: u16) -> Result<u16, RegisterError> {
        let register = RegisterStorage::get_register(reg_location)?;

        info!("loading instruction from register: {register:?}");
        Ok(self.locations[register as usize])
    }

    fn store(&mut self, instr: u16, reg_location: u16) -> Result<(), RegisterError> {
        let register = RegisterStorage::get_register(reg_location)?;

        info!("storing instruction to register: {register:?}");
        self.locations[register as usize] = instr;
        Ok(())
    }

    fn update_flags(&mut self, destination_register: u16) -> Result<(), RegisterError> {
        let result = self.load(destination_register)?;
        if result == 0 {
            let _set_zero_conditional_flag =
                self.store(ConditionalFlags::ZRO as u16, Registers::COND as u16)?;
        } else if result >> 15 == 1 {
            /* a 1 in the left-most bit indicates negative */
            let _set_negative_condition =
                self.store(ConditionalFlags::NEG as u16, Registers::COND as u16)?;
        } else {
            let _set_positive_condition =
                self.store(ConditionalFlags::POS as u16, Registers::COND as u16)?;
        }
        Ok(())
    }
}

impl RegisterStorage {
    pub fn get_register(register: u16) -> Result<Registers, RegisterError> {
        match register {
            0 => Ok(Registers::R0),
            1 => Ok(Registers::R1),
            2 => Ok(Registers::R2),
            3 => Ok(Registers::R3),
            4 => Ok(Registers::R4),
            5 => Ok(Registers::R5),
            6 => Ok(Registers::R6),
            7 => Ok(Registers::R7),
            8 => Ok(Registers::PC),
            9 => Ok(Registers::COND),
            _ => Err(RegisterError::InvalidRegister(register)),
        }
    }
}

pub trait MemomryTrait {
    fn new() -> Self;
    fn read(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, value: u16);
}

#[derive(Debug)]
pub struct Memory {
    pub locations: [u16; MEMORY_MAX],
}

impl MemomryTrait for Memory {
    fn new() -> Self {
        info!("initializing new memory");

        Self {
            locations: [0u16; MEMORY_MAX],
        }
    }

    fn read(&self, memory_address: u16) -> u16 {
        info!("reading from memory address: {memory_address:?}");
        self.locations[memory_address as usize]
    }

    fn write(&mut self, memory_address: u16, value: u16) {
        info!("storing to {value} memory address: {memory_address:?}");
        self.locations[memory_address as usize] = value
    }
}
