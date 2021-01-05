//! CHIP-8 is an interpreted programming language. Chip-8 programs are typically run on CHIP-8
//! virtual machines

#![deny(missing_docs)]

const MEM_MAP: [(u16, u16); 3] = [
    (0x000, 0x1FF), // Chip 8 interpreter (contains font set in emu)
    (0x050, 0x0A0), // Used for the built in 4x5 pixel font set (0-F)
    (0x200, 0xFFF), // Program ROM and work RAM
];

/// This struct represents the CPU structure of CHIP-8 systems
struct CPU {
    /// Current 2-byte opcode
    opcode: u16,
    /// 4K memory
    mem: [u8; 4096],
    /// Fifteen 8-bit general purpose registers, the 16th register is used as a "carry flag"
    regs: [u8; 16],
    /// Index register (0x000-0xFFF)
    idx: u16,
    /// Interupts timer register
    delay_timer: u16,
    /// Hardware timer register
    sound_timer: u16,
    /// Program counter (0x000-0xFFF)
    pc: u16,
    /// Sixteen-level stack
    stack: [u16; 16],
    /// Stack pointer
    sp: u16,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
