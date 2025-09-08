use thiserror::Error;

use crate::libs::types::{
    ConditionalFlags, MemomryTrait, RegisterStorage, RegisterStorageTrait, Registers,
};

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
    fn ldi(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    fn not(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
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
            let _set_zero_conditional_flag =
                register_storage.store(ConditionalFlags::ZRO as u16, Registers::COND);
        } else if result >> 15 == 1 {
            /* a 1 in the left-most bit indicates negative */
            let _set_negative_condition =
                register_storage.store(ConditionalFlags::NEG as u16, Registers::COND);
        } else {
            let _set_positive_condition =
                register_storage.store(ConditionalFlags::POS as u16, Registers::COND);
        }
        Ok(())
    }

    fn add(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /*
        destination register (DR):
        extract the value at destination register from the instr and store in r0
        */
        let _set_destination_register = register_storage.store((instr >> 9) & 0x7, Registers::R0);

        /*
        first operand (SR1):
        extract the value at source register1 from instr and store in r1
        */
        let _set_source_register_1 = register_storage.store((instr >> 6) & 0x7, Registers::R1);

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5).unwrap();
            let _update_destination_register_imm5 = register_storage.store(
                register_storage.load(Registers::R1).unwrap() + imm5,
                Registers::R0,
            );
        } else {
            let _set_source_reg_2 = register_storage.store(instr & 0x7, Registers::R2);
            let _update_destination_register = register_storage.store(
                register_storage.load(Registers::R1).unwrap()
                    + register_storage.load(Registers::R2).unwrap(),
                Registers::R0,
            );
        }

        let _ = Self::update_flags(register_storage, Registers::R0);

        Ok(())
    }

    fn and(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        let _set_destination_reg = register_storage.store((instr >> 9) & 0x7, Registers::R0);

        /* first operand (SR1) */
        let _set_source_register_1 = register_storage.store((instr >> 6) & 0x7, Registers::R1);

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5).unwrap();
            let _update_destination_reg_with_imm5 = register_storage.store(
                register_storage.load(Registers::R1).unwrap() & imm5,
                Registers::R0,
            );
        } else {
            let _set_source_reg_2 = register_storage.store(instr & 0x7, Registers::R2);

            let _update_destination_reg_without_imm5 = register_storage.store(
                register_storage.load(Registers::R1).unwrap()
                    & register_storage.load(Registers::R2).unwrap(),
                Registers::R0,
            );
        }

        let _ = Self::update_flags(register_storage, Registers::R0);

        Ok(())
    }

    fn ldi(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        let _set_destination_register = register_storage.store((instr >> 9) & 0x7, Registers::R0);

        /* PCoffset9: Extract and sign-extend the immediate value */
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;

        let current_pc = register_storage.load(Registers::PC).unwrap();

        /* Calculate the memory address */
        let mem_address = current_pc + pc_offset;

        let value = memory.read(memory.read(mem_address));

        let _update_destination_reg = register_storage.store(value, Registers::R0);

        let _ = Self::update_flags(register_storage, Registers::R0);

        Ok(())
    }

    fn not(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /* Read content of destination reg from instruction and store to register location R0 */
        let _set_destination_register = register_storage.store((instr >> 9) & 0x7, Registers::R0);
        let _set_source_register_1 = register_storage.store((instr >> 6) & 0x7, Registers::R1);

        let _ = register_storage.store(
            !register_storage.load(Registers::R1).unwrap(),
            Registers::R0,
        );

        let _update_conditional_flag = Self::update_flags(register_storage, Registers::R0);
        Ok(())
    }
}

// 1000000000000000
// 0000-000-001-0-00-000
