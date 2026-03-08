use nes_emulator::cpu::{Flags, CPU};
use std::vec;

/// Asserts that only the Break flag is set (used for tests that don't modify other flags).
/// Note: InteruptDisable and Unused flags are always ignored in assertions since they are
/// managed by CPU initialization and interrupt handling.
#[allow(dead_code)]
pub fn only_break_flag_set(cpu: &CPU) {
    assert_flags(cpu, vec![]);
}

/// Asserts that the given flags are set. Since all tests end with BRK (0x00),
/// the Break flag is automatically included in the expected flags.
/// Note: InteruptDisable and Unused flags are always ignored in assertions.
pub fn assert_flags(cpu: &CPU, enabled_flags: Vec<Flags>) {
    let mut all_flags = enabled_flags;
    if !all_flags.contains(&Flags::Break) {
        all_flags.push(Flags::Break);
    }
    check_flags(cpu, all_flags, true);
}

/// Asserts that the given flags are set (without Break flag).
/// Use this for direct flag checks or when testing before BRK execution.
/// Note: InteruptDisable and Unused flags are always ignored in assertions.
#[allow(dead_code)]
pub fn assert_flags_without_break(cpu: &CPU, enabled_flags: Vec<Flags>) {
    check_flags(cpu, enabled_flags, false);
}

/// Asserts that a single flag is set.
/// Automatically includes the Break flag since all tests end with BRK (0x00).
pub fn assert_flag(cpu: &CPU, flag: Flags) {
    assert_flags(cpu, vec![flag]);
}

/// Asserts that no flags are set (only Break flag, which is automatically added).
pub fn assert_no_flags(cpu: &CPU) {
    assert_flags(cpu, vec![]);
}

/// Internal function to check flag state against expected flags.
/// Ignores InteruptDisable and Unused flags as they are managed by CPU initialization.
fn check_flags(cpu: &CPU, enabled_flags: Vec<Flags>, _expect_break: bool) {
    for f in Flags::all() {
        // These flags are always managed by CPU initialization and interrupts,
        // so they're not tested by operation-specific tests
        if (f == Flags::InteruptDisable) | (f == Flags::Unused) {
            continue;
        }

        let is_enabled = cpu.status.contains(f);
        let should_be_enabled = enabled_flags.contains(&f);

        assert_eq!(
            is_enabled,
            should_be_enabled,
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
