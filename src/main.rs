use chip8::*;
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameResult;
use std::io::Read;

const WINDOW_WIDTH: f32 = CHIP8_SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
const WINDOW_HEIGHT: f32 = CHIP8_SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

fn main() -> GameResult<()> {
    let fpath: String = std::env::args().skip(1).take(1).collect();
    let mut prog = std::fs::File::open(fpath)?;

    let mut prog_mem = [0u8; 0xDFF];
    let prog_len = prog.read(&mut prog_mem)?;

    let mut chip8 = Chip8::default();
    chip8.reset();
    chip8.load(&prog_mem, prog_len);

    let (ctx, event_loop) = &mut ContextBuilder::new("CHIP-8", "Tung L. Vo")
        .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    event::run(ctx, event_loop, &mut chip8)
}
