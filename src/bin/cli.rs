use virtual_machine::libs::{constants::MEMORY_MAX, types::{ConditionalFlags, RegisterStorage, RegisterStorageTrait, Registers}};





fn main() {
    let mut register: RegisterStorage = RegisterStorage::new();

    // since exactly one condition flag should be set at any given time, set the Z flag
    let _ = register.store(ConditionalFlags::ZRO as u16, Registers::COND);
    println!("{register:?}")
}