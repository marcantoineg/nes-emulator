use nes_emulator::cpu::{Flags, CPU};

mod common;
use common::{assert_flag, assert_flags};

macro_rules! generate_0x08_implied_single_flag_test {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
       fn $name () {
            let flag = $value;
            let mut cpu = CPU::new();
            cpu.status.insert(flag);

            cpu.load_and_run_without_reset(vec![0x08, 0x00]);

            assert_flag(&cpu, flag);
            assert_eq!(cpu.memory.read(0x01FF), cpu.status.bits());
        }
    )*
    }
}

generate_0x08_implied_single_flag_test! {
    test_0x08_implied_saves_carry_flag_correctly: Flags::Carry,
    test_0x08_implied_saves_zero_flag_correctly: Flags::Zero,
    test_0x08_implied_saves_interupt_flag_correctly: Flags::InteruptDisable,
    test_0x08_implied_saves_decimal_flag_correctly: Flags::Decimal,
    test_0x08_implied_saves_break_flag_correctly: Flags::Break,
    test_0x08_implied_saves_break2_flag_correctly: Flags::Break2,
    test_0x08_implied_saves_overflow_flag_correctly: Flags::Overflow,
    test_0x08_implied_saves_negative_flag_correctly: Flags::Negative,
}

#[test]
fn test_0x08_implied_saves_multiple_flags_correctly() {
    let mut cpu = CPU::new();
    cpu.status
        .insert(Flags::Carry | Flags::Zero | Flags::Negative);
    println!("status: {}", cpu.status.bits());

    cpu.load_and_run_without_reset(vec![0x08, 0x00]);

    assert_flags(&cpu, vec![Flags::Carry, Flags::Zero, Flags::Negative]);
    assert_eq!(
        cpu.memory.read(0x01FF),
        (Flags::Carry | Flags::Zero | Flags::Negative | Flags::Init).bits()
    );
}
