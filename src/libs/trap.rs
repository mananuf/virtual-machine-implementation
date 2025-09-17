use std::io::{self, Read, Write};

use crate::libs::types::{
    MemomryTrait, RegisterError, RegisterStorage, RegisterStorageTrait, Registers,
};

#[derive(Debug)]
pub enum Trap {
    GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    OUT = 0x21,   /* output a character */
    PUTS = 0x22,  /* output a word string */
    IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PUTSP = 0x24, /* output a byte string */
    HALT = 0x25,  /* halt the program */
}

pub trait TrapTrait {
    fn getc(register_storage: &mut RegisterStorage) -> Result<(), RegisterError>;
    fn out(register_storage: &mut RegisterStorage) -> Result<(), RegisterError>;
    fn puts(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
    ) -> Result<(), RegisterError>;
    fn trap_in(register_storage: &mut RegisterStorage) -> Result<(), RegisterError>;
    fn putsp(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
    ) -> Result<(), RegisterError>;
    fn halt() -> Result<(), RegisterError>;
}

impl TrapTrait for Trap {
    fn getc(register_storage: &mut RegisterStorage) -> Result<(), RegisterError> {
        // Read a single character from stdin (non-blocking)
        let mut buffer = [0u8; 1];
        let char_code = if io::stdin().read_exact(&mut buffer).is_ok() {
            buffer[0] as u16
        } else {
            // No input available, return 0 or handle appropriately
            0
        };

        // Store in R0 and clear high 8 bits
        register_storage.store(char_code & 0x00FF, Registers::R0 as u16)?;

        register_storage.update_flags(Registers::R0 as u16)?;

        Ok(())
    }

    fn out(register_storage: &mut RegisterStorage) -> Result<(), RegisterError> {
        let char_code = register_storage.load(Registers::R0 as u16)? & 0x00FF;

        // Output character to stdout
        if let Some(c) = char::from_u32(char_code as u32) {
            print!("{}", c);
            io::stdout().flush().unwrap();
        }

        Ok(())
    }

    fn puts(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
    ) -> Result<(), RegisterError> {
        let mut address = register_storage.load(Registers::R0 as u16)?;

        // Read and output characters until null terminator 0x0000
        loop {
            let memory_value = memory.read(address);

            if memory_value == 0 {
                break;
            }

            let char_code = memory_value & 0x00FF;
            if let Some(c) = char::from_u32(char_code as u32) {
                print!("{}", c);
            }

            address += 1;
        }

        io::stdout().flush().unwrap();
        Ok(())
    }

    fn trap_in(register_storage: &mut RegisterStorage) -> Result<(), RegisterError> {
        print!("Enter a character: ");
        io::stdout().flush().unwrap();

        // Read character
        let mut buffer = [0u8; 1];
        let char_code = if io::stdin().read_exact(&mut buffer).is_ok() {
            buffer[0] as u16
        } else {
            0
        };

        // Store in R0 and echo
        register_storage.store(char_code & 0x00FF, Registers::R0 as u16)?;
        if let Some(c) = char::from_u32(char_code as u32) {
            println!("{}", c);
        }

        register_storage.update_flags(Registers::R0 as u16)?;
        Ok(())
    }

    fn putsp(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
    ) -> Result<(), RegisterError> {
        let mut address = register_storage.load(Registers::R0 as u16)?;

        // Read and output packed characters until null terminator 0x000
        loop {
            let memory_value = memory.read(address);
            if memory_value == 0 {
                break;
            }

            // Output both bytes (little-endian packed chars)
            let char1 = (memory_value & 0x00FF) as u8;
            let char2 = ((memory_value >> 8) & 0x00FF) as u8;

            if let Some(c) = char::from_u32(char1 as u32) {
                print!("{}", c);
            }
            if char2 != 0 {
                if let Some(c) = char::from_u32(char2 as u32) {
                    print!("{}", c);
                }
            }

            address += 1;
        }

        io::stdout().flush().unwrap();
        Ok(())
    }

    fn halt() -> Result<(), RegisterError> {
        println!("Program halted");
        std::process::exit(0);
    }
}

impl Trap {
    pub fn execute_trap_instruction(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        trap_vector: u16,
    ) -> Result<(), RegisterError> {
        match Self::from_u16(trap_vector & 0xFF) {
            Some(Trap::GETC) => Self::getc(register_storage),
            Some(Trap::OUT) => Self::out(register_storage),
            Some(Trap::PUTS) => Self::puts(register_storage, memory),
            Some(Trap::IN) => Self::trap_in(register_storage),
            Some(Trap::PUTSP) => Self::putsp(register_storage, memory),
            Some(Trap::HALT) => Self::halt(),
            _ => todo!(),
        }
    }

    pub fn from_u16(instr: u16) -> Option<Trap> {
        match instr {
            0x20 => Some(Trap::GETC),
            0x21 => Some(Trap::OUT),
            0x22 => Some(Trap::PUTS),
            0x23 => Some(Trap::IN),
            0x24 => Some(Trap::PUTSP),
            0x25 => Some(Trap::HALT),
            _ => None,
        }
    }
}
