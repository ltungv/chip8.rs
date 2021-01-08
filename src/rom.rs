use std::io::Read;

/// This struct represents a program that can be put into chip-8 memory
pub struct Rom {
    /// Data of the program
    pub data: [u8; 0xDFF],
}

impl Rom {
    /// Create a new rom with data read from the given file
    pub fn new(fpath: &str) -> std::io::Result<Self> {
        let mut f = std::fs::File::open(fpath).expect("file not found");
        let mut data = [0u8; 0xDFF];
        let _ = f.read(&mut data)?;
        Ok(Self { data })
    }
}
