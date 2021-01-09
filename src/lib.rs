//! CHIP-8 is an interpreted programming language. Chip-8 programs are typically run on CHIP-8
//! virtual machines

#![deny(missing_docs)]

use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use rand::prelude::*;
use std::time;

/// Screen width of chip-8
pub const CHIP8_SCREEN_WIDTH: usize = 64;
/// Screen height of chip-8
pub const CHIP8_SCREEN_HEIGHT: usize = 32;
/// Size of each pixel when render to the host machine
pub const PIXEL_SIZE: i32 = 16;

/// This struct represents the CPU structure of CHIP-8 systems
pub struct Chip8 {
    /// Index register (0x000-0xFFF)
    i: u16,
    /// Program counter (0x000-0xFFF)
    pc: u16,
    /// Stack pointer
    sp: u8,
    /// Delay timer register
    dt: u8,
    /// Sound timer register
    st: u8,
    /// Fifteen 8-bit general purpose registers, the 16th register is used as a "carry flag"
    v: [u8; 16],
    /// 4K memory
    /// - (0, 512): Chip 8 interpreter (contains font set in emulator)
    /// - (512, 4096): Chip 8 program
    mem: [u8; 4096],
    /// Sixteen-level stack
    stack: [u16; 16],
    /// Graphics system, one instruction is used the draw sprite to the
    /// screen; drawing is done in XOR mode, VF register is set if a
    /// pixel is turned off.
    gfx: [bool; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT],
    /// Current state of the HEX-based keypad
    key: [bool; 16],
    /// True of the graphics memory is recently updated
    gfx_updated: bool,
    timing: time::Instant,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            mem: [0; 4096],
            stack: [0; 16],
            gfx: [false; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT],
            key: [false; 16],
            gfx_updated: false,
            timing: time::Instant::now(),
        }
    }
}

impl EventHandler for Chip8 {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const TICKS_PER_SEC: u32 = 1000;
        while timer::check_update_time(ctx, TICKS_PER_SEC) {
            self.tick();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.gfx_updated {
            self.gfx_updated = false;
            graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
            for y in 0..CHIP8_SCREEN_HEIGHT {
                for x in 0..CHIP8_SCREEN_WIDTH {
                    if self.gfx[x + y * CHIP8_SCREEN_WIDTH] {
                        let rect = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            Rect::new_i32(
                                x as i32 * PIXEL_SIZE,
                                y as i32 * PIXEL_SIZE,
                                PIXEL_SIZE,
                                PIXEL_SIZE,
                            ),
                            (1.0, 1.0, 1.0, 1.0).into(),
                        )?;
                        graphics::draw(ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                    }
                }
            }
            graphics::present(ctx)?;
        }
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Key1 => {
                self.key[0x1] = true;
            }
            KeyCode::Key2 => {
                self.key[0x2] = true;
            }
            KeyCode::Key3 => {
                self.key[0x3] = true;
            }
            KeyCode::Key4 => {
                self.key[0xC] = true;
            }
            KeyCode::Q => {
                self.key[0x4] = true;
            }
            KeyCode::W => {
                self.key[0x5] = true;
            }
            KeyCode::E => {
                self.key[0x6] = true;
            }
            KeyCode::R => {
                self.key[0xD] = true;
            }
            KeyCode::A => {
                self.key[0x7] = true;
            }
            KeyCode::S => {
                self.key[0x8] = true;
            }
            KeyCode::D => {
                self.key[0x9] = true;
            }
            KeyCode::F => {
                self.key[0xE] = true;
            }
            KeyCode::Z => {
                self.key[0xA] = true;
            }
            KeyCode::X => {
                self.key[0x0] = true;
            }
            KeyCode::C => {
                self.key[0xB] = true;
            }
            KeyCode::V => {
                self.key[0xF] = true;
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Key1 => {
                self.key[0x1] = false;
            }
            KeyCode::Key2 => {
                self.key[0x2] = false;
            }
            KeyCode::Key3 => {
                self.key[0x3] = false;
            }
            KeyCode::Key4 => {
                self.key[0xC] = false;
            }
            KeyCode::Q => {
                self.key[0x4] = false;
            }
            KeyCode::W => {
                self.key[0x5] = false;
            }
            KeyCode::E => {
                self.key[0x6] = false;
            }
            KeyCode::R => {
                self.key[0xD] = false;
            }
            KeyCode::A => {
                self.key[0x7] = false;
            }
            KeyCode::S => {
                self.key[0x8] = false;
            }
            KeyCode::D => {
                self.key[0x9] = false;
            }
            KeyCode::F => {
                self.key[0xE] = false;
            }
            KeyCode::Z => {
                self.key[0xA] = false;
            }
            KeyCode::X => {
                self.key[0x0] = false;
            }
            KeyCode::C => {
                self.key[0xB] = false;
            }
            KeyCode::V => {
                self.key[0xF] = false;
            }
            _ => (),
        }
    }
}

impl Chip8 {
    /// Set the state of the system to the intial state
    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200; // program begins at 0x200
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.v = [0; 16];
        self.mem = [0; 4096];
        self.stack = [0; 16];
        self.gfx = [false; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT]; // clear display
        self.key = [false; 16]; // clear display
        self.gfx_updated = false;
        self.timing = time::Instant::now();
        // Load font sprites to the first 80 bytes of the memory.
        // The first four nibble is used to determine what the character is
        [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]
        .iter()
        .enumerate()
        .for_each(|(i, b)| self.mem[i] = *b);
    }

    /// Load the bytes from the file at the given path into memory
    pub fn load(&mut self, prog_mem: &[u8; 0xDFF], prog_len: usize) {
        self.mem[0x200..0x200 + prog_len].copy_from_slice(&prog_mem[..prog_len]);
    }

    fn tick(&mut self) {
        // Get and process the opcode
        let opcode = self.fetch();
        self.pc = match self.exec(Inst::from(opcode)) {
            Flow::Halt => self.pc - 2,
            Flow::Next => self.pc,
            Flow::Skip => self.pc + 2,
            Flow::Jump(addr) => addr,
        };
        // Update timers
        // The two timers count down to zero if they have been set to a
        // value larger than zero (counting at 60Hz).
        if self.timing.elapsed() >= time::Duration::from_millis(20) {
            self.timing = time::Instant::now();
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                if self.st == 1 {
                    println!("BEEP");
                }
                self.st -= 1;
            }
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        self.pc += 2;
        (self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16
    }

    fn exec(&mut self, inst: Inst) -> Flow {
        match inst {
            Inst::Op00E0 => {
                self.gfx_updated = true;
                self.gfx.iter_mut().for_each(|pixel| *pixel = false);
            }
            Inst::Op00EE => {
                self.sp -= 1;
                return Flow::Jump(self.stack[self.sp as usize]);
            }
            Inst::Op1NNN(nnn) => return Flow::Jump(nnn),
            Inst::Op2NNN(nnn) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                return Flow::Jump(nnn);
            }
            Inst::Op3XKK(x, kk) => {
                if self.v[x] == kk {
                    return Flow::Skip;
                }
            }
            Inst::Op4XKK(x, kk) => {
                if self.v[x] != kk {
                    return Flow::Skip;
                }
            }
            Inst::Op5XY0(x, y) => {
                if self.v[x] == self.v[y] {
                    return Flow::Skip;
                }
            }
            Inst::Op6XKK(x, kk) => self.v[x] = kk,
            Inst::Op7XKK(x, kk) => self.v[x] = self.v[x].wrapping_add(kk),
            Inst::Op8XY0(x, y) => self.v[x] = self.v[y],
            Inst::Op8XY1(x, y) => self.v[x] |= self.v[y],
            Inst::Op8XY2(x, y) => self.v[x] &= self.v[y],
            Inst::Op8XY3(x, y) => self.v[x] ^= self.v[y],
            Inst::Op8XY4(x, y) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = if overflow { 1 } else { 0 };
                self.v[x] = res;
            }
            Inst::Op8XY5(x, y) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            }
            Inst::Op8XY6(x, _y) => {
                self.v[0xF] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            }
            Inst::Op8XY7(x, y) => {
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res;
            }
            Inst::Op8XYE(x, _y) => {
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            Inst::Op9XY0(x, y) => {
                if self.v[x] != self.v[y] {
                    return Flow::Skip;
                }
            }
            Inst::OpANNN(nnn) => self.i = nnn,
            Inst::OpBNNN(nnn) => return Flow::Jump(self.v[0] as u16 + nnn),
            Inst::OpCXKK(x, kk) => self.v[x] = random::<u8>() & kk,
            Inst::OpDXYN(x, y, n) => {
                self.gfx_updated = true;
                self.v[0xF] = 0;
                for (y_offset, sprite) in self.mem[self.i as usize..(self.i + n) as usize]
                    .iter()
                    .enumerate()
                {
                    let y_screen = (self.v[y] as usize + y_offset) % CHIP8_SCREEN_HEIGHT;
                    for x_offset in 0..8 {
                        let x_screen = (self.v[x] as usize + x_offset) % CHIP8_SCREEN_WIDTH;
                        if (sprite & (0x80 >> x_offset)) != 0 {
                            if self.gfx[x_screen + y_screen * CHIP8_SCREEN_WIDTH] {
                                self.v[0xF] = 1;
                            }
                            self.gfx[x_screen + y_screen * CHIP8_SCREEN_WIDTH] ^= true;
                        }
                    }
                }
            }
            Inst::OpEX9E(x) => {
                if self.key[self.v[x] as usize] {
                    return Flow::Skip;
                }
            }
            Inst::OpEXA1(x) => {
                if !self.key[self.v[x] as usize] {
                    return Flow::Skip;
                }
            }
            Inst::OpFX07(x) => self.v[x] = self.dt,
            Inst::OpFX0A(x) => {
                let mut pressed = false;
                for (key_idx, key_pressed) in self.key.iter().enumerate() {
                    if *key_pressed {
                        self.v[x] = key_idx as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    return Flow::Halt;
                }
            }
            Inst::OpFX15(x) => self.dt = self.v[x],
            Inst::OpFX18(x) => self.st = self.v[x],
            Inst::OpFX1E(x) => self.i = self.i.wrapping_add(self.v[x] as u16),
            Inst::OpFX29(x) => self.i = self.v[x] as u16 * 5,
            Inst::OpFX33(x) => {
                self.mem[self.i as usize] = self.v[x] / 100;
                self.mem[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.mem[self.i as usize + 2] = (self.v[x] % 100) % 10;
            }
            Inst::OpFX55(x) => {
                self.mem[self.i as usize..=self.i as usize + x].copy_from_slice(&self.v[0..=x]);
                self.i += x as u16 + 1;
            }
            Inst::OpFX65(x) => {
                self.v[0..=x].copy_from_slice(&self.mem[self.i as usize..=self.i as usize + x]);
                self.i += x as u16 + 1;
            }
        }
        Flow::Next
    }
}

enum Flow {
    Halt,
    Next,
    Skip,
    Jump(u16),
}

#[derive(Debug)]
enum Inst {
    /// 00E0 - CLS
    /// Clear the display.
    Op00E0,
    /// 00EE - RET
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    Op00EE,
    /// 1NNN - JP addr
    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    Op1NNN(u16),
    /// 2NNN - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    Op2NNN(u16),
    /// 3XKK - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    Op3XKK(usize, u8),
    /// 4XKK - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    Op4XKK(usize, u8),
    /// 5XY0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    Op5XY0(usize, usize),
    /// 6XKK - LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    Op6XKK(usize, u8),
    /// 7XKK - ADD Vx, byte
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    Op7XKK(usize, u8),
    /// 8XY0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    Op8XY0(usize, usize),
    /// 8XY1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits
    /// from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    Op8XY1(usize, usize),
    /// 8XY2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits
    /// from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    Op8XY2(usize, usize),
    /// 8XY3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares
    /// the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result
    /// is set to 1. Otherwise, it is 0.
    Op8XY3(usize, usize),
    /// 8XY4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    Op8XY4(usize, usize),
    /// 8XY5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    Op8XY5(usize, usize),
    /// 8XY6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    Op8XY6(usize, usize),
    /// 8XY7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    Op8XY7(usize, usize),
    /// 8XYE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    Op8XYE(usize, usize),
    /// 9XY0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    Op9XY0(usize, usize),
    /// ANNN - ld i, addr
    /// set i = nnn.
    /// the value of register i is set to nnn.
    OpANNN(u16),
    /// BNNN - JP V0, addr
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    OpBNNN(u16),
    /// CXKK - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
    /// See instruction 8xy2 for more information on AND.
    OpCXKK(usize, u8),
    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprite
    /// on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4,
    /// Display, for more information on the Chip-8 screen and sprites.
    OpDXYN(usize, usize, u16),
    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    OpEX9E(usize),
    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    OpEXA1(usize),
    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    OpFX07(usize),
    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    OpFX0A(usize),
    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    OpFX15(usize),
    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    OpFX18(usize),
    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    OpFX1E(usize),
    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display,
    /// for more information on the Chip-8 hexadecimal font.
    OpFX29(usize),
    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit
    /// at location I+1, and the ones digit at location I+2.
    OpFX33(usize),
    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    OpFX55(usize),
    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    OpFX65(usize),
}

impl From<u16> for Inst {
    fn from(opcode: u16) -> Self {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Self::Op00E0,
            (0x0, 0x0, 0xE, 0xE) => Self::Op00EE,
            (0x1, _, _, _) => Self::Op1NNN(nnn),
            (0x2, _, _, _) => Self::Op2NNN(nnn),
            (0x3, _, _, _) => Self::Op3XKK(x, kk),
            (0x4, _, _, _) => Self::Op4XKK(x, kk),
            (0x5, _, _, 0x0) => Self::Op5XY0(x, y),
            (0x6, _, _, _) => Self::Op6XKK(x, kk),
            (0x7, _, _, _) => Self::Op7XKK(x, kk),
            (0x8, _, _, 0x0) => Self::Op8XY0(x, y),
            (0x8, _, _, 0x1) => Self::Op8XY1(x, y),
            (0x8, _, _, 0x2) => Self::Op8XY2(x, y),
            (0x8, _, _, 0x3) => Self::Op8XY3(x, y),
            (0x8, _, _, 0x4) => Self::Op8XY4(x, y),
            (0x8, _, _, 0x5) => Self::Op8XY5(x, y),
            (0x8, _, _, 0x6) => Self::Op8XY6(x, y),
            (0x8, _, _, 0x7) => Self::Op8XY7(x, y),
            (0x8, _, _, 0xE) => Self::Op8XYE(x, y),
            (0x9, _, _, 0x0) => Self::Op9XY0(x, y),
            (0xA, _, _, _) => Self::OpANNN(nnn),
            (0xB, _, _, _) => Self::OpBNNN(nnn),
            (0xC, _, _, _) => Self::OpCXKK(x, kk),
            (0xD, _, _, _) => Self::OpDXYN(x, y, n),
            (0xE, _, 0x9, 0xE) => Self::OpEX9E(x),
            (0xE, _, 0xA, 0x1) => Self::OpEXA1(x),
            (0xF, _, 0x0, 0x7) => Self::OpFX07(x),
            (0xF, _, 0x0, 0xA) => Self::OpFX0A(x),
            (0xF, _, 0x1, 0x5) => Self::OpFX15(x),
            (0xF, _, 0x1, 0x8) => Self::OpFX18(x),
            (0xF, _, 0x1, 0xE) => Self::OpFX1E(x),
            (0xF, _, 0x2, 0x9) => Self::OpFX29(x),
            (0xF, _, 0x3, 0x3) => Self::OpFX33(x),
            (0xF, _, 0x5, 0x5) => Self::OpFX55(x),
            (0xF, _, 0x6, 0x5) => Self::OpFX65(x),
            (_, _, _, _) => panic!("Opcode is not supported {:#04X}", opcode),
        }
    }
}
