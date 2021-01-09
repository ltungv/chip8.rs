use chip8::*;
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameResult;

const WINDOW_WIDTH: f32 = CHIP8_SCREEN_WIDTH as f32 * PIXEL_SIZE as f32;
const WINDOW_HEIGHT: f32 = CHIP8_SCREEN_HEIGHT as f32 * PIXEL_SIZE as f32;

fn main() -> GameResult<()> {
    let fpath: String = std::env::args().skip(1).take(1).collect();
    let mut chip8 = Chip8::default();
    chip8.reset();
    chip8.load(&fpath)?;

    let (mut ctx, mut event_loop) = ContextBuilder::new("CHIP-8", "Tung L. Vo")
        .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    event::run(&mut ctx, &mut event_loop, &mut chip8)
}
