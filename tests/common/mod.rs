use nes_emulator::cpu::{Flags, CPU};
use std::vec;

#[allow(dead_code)]
pub fn assert_no_flags(cpu: &CPU) {
    assert_flags(cpu, vec![]);
}

#[allow(dead_code)]
pub fn assert_flag(cpu: &CPU, enabled_flag: Flags) {
    assert_flags(cpu, vec![enabled_flag]);
}

#[allow(dead_code)]
pub fn assert_flags(cpu: &CPU, enabled_flags: Vec<Flags>) {
    for f in Flags::all() {
        if (f == Flags::InteruptDisable) | (f == Flags::Unused) {
            continue;
        }
        assert_eq!(
            cpu.status.contains(f),
            enabled_flags.contains(&f),
            "CPU status: {:#010b} | Failing Flag: {} ({:#010b})",
            cpu.status.bits(),
            get_flag_name(f),
            f.bits()
        )
    }
}

#[allow(dead_code)]
pub fn push_to_stack(cpu: &mut CPU, data: u8) {
    let stack_addr = (0x0100 as u16) + (cpu.stack_pointer as u16);
    cpu.memory.write(stack_addr, data);
    cpu.stack_pointer -= 1;
}

fn get_flag_name(f: Flags) -> &'static str {
    match f {
        Flags::Carry => "Carry",
        Flags::Zero => "Zero",
        Flags::InteruptDisable => "Interupt Disable",
        Flags::Decimal => "Decimal",
        Flags::Break => "Break",
        Flags::Overflow => "Overflow",
        Flags::Negative => "Negative",
        _ => "Unhandled Flag",
    }
}

#[allow(dead_code)]
pub fn print_memory_dump(cpu: &CPU) {
    print!("\nmemory hex dump:\n");
    let hex_dump = cpu.memory.dump();
    for hex in hex_dump {
        print!("{:0>2X} ", hex);
    }
    print!("\n\n");
}
