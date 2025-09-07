use std::u16;

use virtual_machine::libs::{
    constants::{MEMORY_MAX, PC_START},
    instructions::{InstructionSet, Instructions},
    types::{
        ConditionalFlags, MemomryTrait, Memory, Opcodes, RegisterStorage, RegisterStorageTrait,
        Registers,
    },
};

fn main() {
    let mut register_storage: RegisterStorage = RegisterStorage::new();
    let mut memory: Memory = Memory::new();

    // since exactly one condition flag should be set at any given time, set the Z flag
    let _set_initial_cond_to_zero =
        register_storage.store(ConditionalFlags::ZRO as u16, Registers::COND);

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let _set_default_mem_addr_in_pc = register_storage.store(PC_START, Registers::PC);

    // loop {
    //     /* FETCH */
    //     let instr = memory.memory_read(&mut register_storage).unwrap();
    //     let op = instr >> 12;

    //     match Opcodes::from_u16(op) {
    //         Some(Opcodes::ADD) => {
    //             println!("Add Op");
    //             break;
    //         },
    //         Some(Opcodes::AND) => {
    //             println!("And Op");
    //             break;
    //         },
    //         Some(Opcodes::NOT) => {
    //             println!("NOT Op");
    //             break;
    //         },
    //         Some(Opcodes::BR) => {
    //             println!("BR Op");
    //             break;
    //         },
    //         Some(Opcodes::JMP) => {
    //             println!("JMP Op");
    //             break;
    //         },
    //         Some(Opcodes::JSR) => {
    //             println!("JSR Op");
    //             break;
    //         },
    //         Some(Opcodes::LD) => {
    //             println!("LD Op");
    //             break;
    //         },
    //         Some(Opcodes::LDI) => {
    //             println!("LDI Op");
    //             break;
    //         },
    //         Some(Opcodes::LDR) => {
    //             println!("LDR Op");
    //             break;
    //         },
    //         Some(Opcodes::LEA) => {
    //             println!("LEA Op");
    //             break;
    //         },
    //         Some(Opcodes::ST) => {
    //             println!("ST Op");
    //             break;
    //         },
    //         Some(Opcodes::STI) => {
    //             println!("STI Op");
    //             break;
    //         },
    //         Some(Opcodes::STR) => {
    //             println!("STR Op");
    //             break;
    //         },
    //         Some(Opcodes::TRAP) => {
    //             println!("TRAP Op");
    //             break;
    //         },
    //         Some(Opcodes::RES) | Some(Opcodes::RTI) | _ => {
    //             println!("Bad Opcode");
    //             break;
    //         }
    //     }
    // }

    // let r0 = (instr >> 9)  & 0x7;
    // let r1 = (instr >> 6)  & 0x7;
    // let imm_flag = (instr >> 5)  & 0x1;
    // println!("{instr :#01x} {r0} {r1} {imm_flag}");
    let bit_count: u32 = 5;
    let bits = 0b01011;
    let sign_extend_result = Instructions::sign_extend(bits, bit_count).unwrap();

    let a = 0b1000000000000000;
    println!("{a}, {:b} {:b}", 0x7, a >> 9 & 0x7);

    println!("{:b}", sign_extend_result);
    println!("{register_storage:?}")
}
