use nes_emulator::cpu::{Flags, CPU};

mod common;

macro_rules! generate_0x28_plp_implied_single_flag_test {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
       fn $name () {
            let flag = $value;
            let mut cpu = CPU::new();
            common::push_to_stack(&mut cpu, flag.bits());


            cpu.load_and_run_without_reset(vec![0x28, 0x00]);

            assert_eq!(cpu.memory.read(0x01FF), 0x00);
            assert_eq!(cpu.status.bits(), flag.bits());
        }
    )*
    }
}

generate_0x28_plp_implied_single_flag_test! {
    test_0x28_plp_imlied_carry_flag: Flags::Carry,
    test_0x28_plp_imlied_zero_flag: Flags::Zero,
    test_0x28_plp_imlied_interupt_flag: Flags::InteruptDisable,
    test_0x28_plp_imlied_decimal_flag: Flags::Decimal,
    test_0x28_plp_imlied_break_flag: Flags::Break,
    test_0x28_plp_imlied_overflow_flag: Flags::Overflow,
    test_0x28_plp_imlied_negative_flag: Flags::Negative,
}

#[test]
fn test_0x28_plp_implied_pulls_multiple_flags_correcly() {
    let mut cpu = CPU::new();
    let expected_flags =
        (Flags::Zero | Flags::Carry | Flags::Overflow | Flags::InteruptDisable).bits();
    common::push_to_stack(&mut cpu, expected_flags);

    cpu.load_and_run_without_reset(vec![0x28, 0x00]);

    assert_eq!(cpu.memory.read(0x01FF), 0x00);
    assert_eq!(cpu.status.bits(), expected_flags);
}
