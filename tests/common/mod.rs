use std::vec;
use nes_emulator::cpu::{CPU, Flags};

pub fn assert_no_flags(cpu: &CPU) {
    assert_flags(cpu, vec![]);
}

pub fn assert_flag(cpu: &CPU, enabled_flag: Flags) {
    assert_flags(cpu, vec![enabled_flag]);
}

pub fn assert_flags(cpu: &CPU, enabled_flags: Vec<Flags>) {
    for f in Flags::all() {
        if (f == Flags::Break) | (f == Flags::Break2) | (f == Flags::Init) {
            continue;
        }
        assert_eq!(
            cpu.status.contains(f), enabled_flags.contains(&f),
            "CPU status: {:#010b} | Failing Flag: {} ({:#010b})", cpu.status.bits(), get_flag_name(f), f.bits()
        )
    }
}

fn get_flag_name(f: Flags) -> &'static str {
    match f {
        Flags::Init => "Init",
        Flags::Carry => "Carry",
        Flags::Zero => "Zero",
        Flags::InteruptDisable => "Interupt Disable",
        Flags::Decimal => "Decimal",
        Flags::Break => "Break",
        Flags::Break2 => "Break 2",
        Flags::Overflow => "Overflow",
        Flags::Negative => "Negative",
        _ => "Unhandled Flag"
    }
}