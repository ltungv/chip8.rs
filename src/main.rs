use chip8::*;

fn main() {
    let mut chip8 = Chip8::default();
    chip8.reset();
    loop {
        chip8.cycle();
    }
}
