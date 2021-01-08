use chip8::rom::*;
use chip8::*;

fn main() {
    let fpath: String = std::env::args().skip(1).take(1).collect();
    let rom = Rom::new(&fpath).unwrap();

    let mut chip8 = Chip8::default();
    chip8.reset();
    chip8.load(&rom.data);
    for _ in 0..32 {
        chip8.tick();
    }
}
