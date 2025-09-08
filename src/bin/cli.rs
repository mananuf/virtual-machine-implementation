use std::{process::abort, u16};

use tracing::info;
use virtual_machine::libs::{
    constants::{MEMORY_MAX, PC_START},
    instructions::{InstructionSet, Instructions},
    types::{
        ConditionalFlags, MemomryTrait, Memory, Opcodes, RegisterStorage, RegisterStorageTrait,
        Registers,
    },
};

fn main() {
    tracing_subscriber::fmt::init();
    // RUST_LOG=virtual_machine=trace cargo run --bin cli

    let mut register_storage: RegisterStorage = RegisterStorage::new();
    let mut memory: Memory = Memory::new();

    // since exactly one condition flag should be set at any given time, set the Z flag
    let _set_initial_cond_to_zero =
        register_storage.store(ConditionalFlags::ZRO as u16, Registers::COND);

    /* set the PC to starting position */
    /* 0x3000 is the default */
    let _set_default_mem_addr_in_pc = register_storage.store(PC_START, Registers::PC);

    // loop {
    /* FETCH: Get the instruction from memory at the address pointed to by the PC */
    // let pc = register_storage.load(Registers::PC).unwrap();
    // let instr = memory.read(pc);

    /* INCREMENT PC */
    // register_storage.store(pc + 1, Registers::PC)?;

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
    // abort();
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

    println!("{:016b}", sign_extend_result);
    println!("{:04x}", 0b0000111000000000);
    println!("{:04x}", 0b0000000111111111);
    println!("{:04x}", 0b0000000000000111);

    println!("{:b}", sign_extend_result);
    println!("{register_storage:?}")
}
