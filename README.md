# nes-emulator 🕹️
[![codecov](https://codecov.io/gh/marcantoineg/nes-emulator/graph/badge.svg?token=PA73YKCDNA)](https://codecov.io/gh/marcantoineg/nes-emulator)
<img width="300" align=right src="https://github.com/marcantoineg/nes-emulator/assets/16008095/8e0b5bdf-4387-44b1-b9c3-72b49f40d91a">

_⚠️ this is a Work In Progress_

A (somewhat) simple emulator of the [Nintendo Entertainment System](https://en.wikipedia.org/wiki/Nintendo_Entertainment_System) in [Rust](https://www.rust-lang.org/)

## Development & Testing

### Running Tests
To run all tests:
```bash
cargo test
```

To run tests for a specific operation (e.g., LDA):
```bash
cargo test lda
```

To run tests with output:
```bash
cargo test -- --nocapture
```

Test files are located in the `tests/` directory and are organized by operation (e.g., `lda_tests.rs`, `adc_tests.rs`, etc.).

## Resources
- [NES emulator tutorial](https://bugzmanov.github.io/nes_ebook/)

**References:**
- [6502 cpu instructions](https://www.nesdev.org/obelisk-6502-guide/reference.html)
- [6502 cpu instructions - II](http://www.6502.org/tutorials/6502opcodes.html)
- [js emu](https://itema-as.github.io/6502js/)