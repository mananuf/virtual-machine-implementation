use thiserror::Error;

use crate::libs::{
    trap::Trap,
    types::{
        ConditionalFlags, MemomryTrait, Memory, RegisterError, RegisterStorage,
        RegisterStorageTrait, Registers,
    },
};

#[derive(Debug, Error)]
pub enum InstructionSetError {
    #[error("Instruction Error: Invalid Bit Count {0}")]
    InvalidBitCount(u32),
    #[error("Register Error: {0}")]
    RegisterError(#[from] RegisterError),
}

pub trait InstructionSet {
    fn sign_extend(bits: u16, bit_count: u32) -> Result<u16, InstructionSetError>;
    fn add(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
    fn and(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
    fn ldi(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    fn not(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
    // /* BR */
    fn branch(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* JMP */
    fn jump(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
    // /* JMPR */
    fn jump_register(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* LD */
    fn load(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* LDR */
    fn load_register(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* LEA */
    fn load_effective_address(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* ST */
    fn store(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* STI */
    fn store_indirect(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* STR */
    fn store_register(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError>;
    // /* RET */
    fn return_from_subroutine(
        register_storage: &mut RegisterStorage,
    ) -> Result<(), InstructionSetError>;
    /* RTI */
    // fn return_from_interrupt(
    //     register_storage: &mut RegisterStorage,
    //     memory: &impl MemomryTrait,
    //     instr: u16,
    // ) -> Result<(), InstructionSetError>;
    fn trap(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError>;
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

    fn add(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /*
        destination register (DR):
        extract the destination register from the instr
        */
        let r0 = (instr >> 9) & 0x7;

        /*
        first operand (SR1):
        extract the source register1 from instr
        */
        let r1 = (instr >> 6) & 0x7;

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5).unwrap();
            let _update_destination_register_imm5 =
                register_storage.store(register_storage.load(r1)? + imm5, r0)?;
        } else {
            let r2 = instr & 0x7;
            let _update_destination_register =
                register_storage.store(register_storage.load(r1)? + register_storage.load(r2)?, r0);
        }

        let _ = register_storage.update_flags(r0);

        Ok(())
    }

    fn and(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        let r0 = (instr >> 9) & 0x7;

        /* first operand (SR1) */
        let r1 = (instr >> 6) & 0x7;

        /* whether we are in immediate mode */
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = Self::sign_extend(instr & 0x1F, 5)?;
            let _update_destination_reg =
                register_storage.store(register_storage.load(r1)? & imm5, r0)?;
        } else {
            let r2 = instr & 0x7;

            let _update_destination_reg = register_storage
                .store(register_storage.load(r1)? & register_storage.load(r2)?, r0)?;
        }

        let _ = register_storage.update_flags(r0);

        Ok(())
    }

    fn ldi(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        /* destination register (DR) */
        let r0 = (instr >> 9) & 0x7;

        /* PCoffset9: Extract and sign-extend the immediate value */
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;

        let current_pc = register_storage.load(Registers::PC as u16)?;

        /* Calculate the memory address */
        let mem_address = current_pc + pc_offset;

        let value = memory.read(memory.read(mem_address));

        let _update_destination_reg = register_storage.store(value, r0);

        let _ = register_storage.update_flags(r0);

        Ok(())
    }

    fn not(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;

        let _ = register_storage.store(!register_storage.load(r1)?, r0);
        let _update_conditional_flag = register_storage.update_flags(r0)?;

        Ok(())
    }

    fn branch(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        /* PCoffset9: Extract and sign-extend the immediate value */
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;
        let cond_flag = (instr >> 9) & 0x7;

        if cond_flag & register_storage.load(Registers::COND as u16)? != 0 {
            register_storage.locations[Registers::PC as usize] += pc_offset;
        }

        Ok(())
    }

    fn jump(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        let r1 = (instr >> 6) & 0x7;
        let _update_pc =
            register_storage.store(register_storage.load(r1)?, Registers::PC as u16)?;
        Ok(())
    }

    fn jump_register(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let pc = register_storage.load(Registers::PC as u16)?;
        let _set_source_register_7 = register_storage.store(pc, Registers::R7 as u16)?;

        if (instr >> 11) & 1 == 1 {
            /* JSR */
            let _update_pc_for_jsr = register_storage.store(
                pc + Self::sign_extend(instr & 0x7FF, 11)?,
                Registers::PC as u16,
            );
        } else {
            /* JSRR */
            let r1 = (instr >> 6) & 0x7;
            let _update_pc_for_jssr =
                register_storage.store(register_storage.load(r1)?, Registers::PC as u16);
        }
        Ok(())
    }

    fn load(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;
        let memory_addr = register_storage.load(Registers::PC as u16)? + pc_offset;
        let _update_destination_register = register_storage.store(memory.read(memory_addr), r0);

        let _ = register_storage.update_flags(r0);

        Ok(())
    }

    fn load_register(
        register_storage: &mut RegisterStorage,
        memory: &impl MemomryTrait,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let offset = Self::sign_extend(instr & 0x3F, 6)?;
        let memory_addr = register_storage.load(r1)? + offset;
        let _update_destination_register = register_storage.store(memory.read(memory_addr), r1)?;

        let _ = register_storage.update_flags(r0);
        Ok(())
    }

    fn load_effective_address(
        register_storage: &mut RegisterStorage,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;
        let _update_destination_reg =
            register_storage.store(register_storage.load(Registers::PC as u16)? + pc_offset, r0);

        let _ = register_storage.update_flags(r0);

        Ok(())
    }

    fn store(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;
        let memory_addr = register_storage.load(Registers::PC as u16)? + pc_offset;

        memory.write(memory_addr, register_storage.load(r0)?);

        Ok(())
    }

    fn store_indirect(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = Self::sign_extend(instr & 0x1FF, 9)?;
        let memory_addr = register_storage.load(Registers::PC as u16)? + pc_offset;

        memory.write(memory.read(memory_addr), register_storage.load(r0)?);
        Ok(())
    }

    fn store_register(
        register_storage: &mut RegisterStorage,
        memory: &mut Memory,
        instr: u16,
    ) -> Result<(), InstructionSetError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let pc_offset = Self::sign_extend(instr & 0x3F, 6)?;
        let memory_addr = register_storage.load(r1)? + pc_offset;

        memory.write(memory_addr, register_storage.load(r0)?);
        Ok(())
    }

    fn return_from_subroutine(
        register_storage: &mut RegisterStorage,
    ) -> Result<(), InstructionSetError> {
        let _update_pc_value_with_r7 = register_storage.store(register_storage.load(Registers::R7 as u16)?, Registers::PC as u16)?;
        Ok(())
    }

    // fn return_from_interrupt(register_storage: &mut RegisterStorage, memory: &impl MemomryTrait, instr: u16) -> Result<(), InstructionSetError> {
    //     if (instr >> 15) & 1 == 0 {
    //         let r6 = register_storage.load(Registers::R6).unwrap();
    //         let _update_pc_with_value_from_memory_at_r6 = register_storage.store(memory.read(r6), Registers::PC);
    //         let _increment_r6_by_1 = register_storage.store(r6 + 1, Registers::R6);
    //         let r6 = register_storage.load(Registers::R6).unwrap();
    //         let TEMP = memory.read(r6);
    //         let _increment_r6_by_1 = register_storage.store(r6 + 1, Registers::R6);
    //     }

    //     Ok(())
    // }

    fn trap(register_storage: &mut RegisterStorage, instr: u16) -> Result<(), InstructionSetError> {
        let _set_register_7 = register_storage.store(register_storage.load(Registers::PC as u16)?, Registers::R7 as u16)?;
        Trap::execute_trap_instruction(register_storage, instr)?;
        Ok(())
    }
}

// 1000000000000000
// 0000-000-001-0-00-000
