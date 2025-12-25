# Ferrix: An OS written in Rust

Inspired by: [Blog OS](os.phil-opp.com) by Philipp Oppermann.

---
## Devlog

### VGA Buffer Implementation
- Implemented a VGA text buffer driver that writes directly to memory address `0xb8000` to display colored text on screen. 
- The `print!` and `println!` macros were overridden to use the custom VGA writer via the [`core::fmt::Write`](https://doc.rust-lang.org/core/fmt/trait.Write.html) trait, enabling formatted text output directly to the screen.

<br>

![alt text](assets/vga_qemu.jpeg)

---