use thiserror::Error;

use crate::libs::types::{ConditionalFlags, RegisterStorage, Registers};

#[derive(Debug, Error)]
pub enum InstructionSetError {
    #[error("Invalid Bit Count: {0}")]
    InvalidBitCount(u32),
}

pub trait InstructionSet {
    fn sign_extend(bits: u16, bit_count: u32) -> Result<u16, InstructionSetError>;
    fn update_flags(
        register_storage: &mut RegisterStorage,
        data_register: Registers,
    ) -> Result<(), InstructionSetError>;
    fn add(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
    fn and(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
}

pub struct Instructions {}

impl InstructionSet for Instructions {
    fn sign_extend(bits: u16, bit_count: u32) -> Result<u16, InstructionSetError> {
        if bit_count == 0 || bit_count > 16 {
            return Err(InstructionSetError::InvalidBitCount(bit_count));
        }

        let sign_bit_position = bit_count - 1;

        if ((bits >> sign_bit_position) & 1) == 1 {
            // Create mask of 1's in the higher bits
            let mask = (0xFFFFu32 << bit_count) as u16;
            Ok(bits | mask)
        } else {
            Ok(bits)
        }
    }

    fn update_flags(
        register_storage: &mut RegisterStorage,
        destination_register: Registers,
    ) -> Result<(), InstructionSetError> {
        let result = register_storage.locations[destination_register as usize];
        if result == 0 {
            register_storage.locations[Registers::COND as usize] = ConditionalFlags::ZRO as u16;
        } else if result >> 15 == 1 {
            /* a 1 in the left-most bit indicates negative */
            register_storage.locations[Registers::COND as usize] = ConditionalFlags::NEG as u16;
        } else {
            register_storage.locations[Registers::COND as usize] = ConditionalFlags::POS as u16;
        }
        Ok(())
    }

    fn add(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        register_storage.locations[Registers::R0 as usize] = (instr >> 9) & 0x7;

        /* first operand (SR1) */
        register_storage.locations[Registers::R1 as usize] = (instr >> 6) & 0x7;

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5).unwrap();
            register_storage.locations[Registers::R0 as usize] =
                register_storage.locations[Registers::R1 as usize] + imm5;
        } else {
            register_storage.locations[Registers::R2 as usize] = instr & 0x7;
            register_storage.locations[Registers::R0 as usize] = register_storage.locations
                [Registers::R1 as usize]
                + register_storage.locations[Registers::R2 as usize];
        }

        let _ = Self::update_flags(register_storage, Registers::R0);

        Ok(())
    }

    fn and(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        register_storage.locations[Registers::R0 as usize] = (instr >> 9) & 0x7;

        /* first operand (SR1) */
        register_storage.locations[Registers::R1 as usize] = (instr >> 6) & 0x7;

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5).unwrap();
            register_storage.locations[Registers::R0 as usize] =
                register_storage.locations[Registers::R1 as usize] & imm5;
        } else {
            register_storage.locations[Registers::R2 as usize] = instr & 0x7;
            register_storage.locations[Registers::R0 as usize] = register_storage.locations
                [Registers::R1 as usize]
                & register_storage.locations[Registers::R2 as usize];
        }

        let _ = Self::update_flags(register_storage, Registers::R0);

        Ok(())
    }
}

// 1000000000000000
// 0000-000-001-0-00-000
