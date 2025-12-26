# Ferrix: An OS written in Rust

Inspired by: [Blog OS](os.phil-opp.com) by Philipp Oppermann.

---
## Devlog

### VGA Buffer Implementation
- Implemented a VGA text buffer driver that writes directly to memory address `0xb8000` to display colored text on screen. 
- The `print!` and `println!` macros were overridden to use the custom VGA writer via the [`core::fmt::Write`](https://doc.rust-lang.org/core/fmt/trait.Write.html) trait, enabling formatted text output directly to the screen.

<br>

![alt text](assets/vga_qemu.jpeg)


### Custom Testing Framework
- Built a custom testing harness using Rust's `custom_test_frameworks` feature, as we're on bare metal.
- Tests communicate results through a serial port interface and automatically exit QEMU with success/failure codes.

<br>

![alt text](assets/test_trivial.jpeg)
---