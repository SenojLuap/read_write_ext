pub trait WriteExt {
    /// Write a [`u16`]
    fn write_u16(&mut self, data: u16) -> Result<()>;

    /// Write a [`u32`]
    fn write_u32(&mut self, data: u32) -> Result<()>;

    /// Write a [`u64`]
    fn write_u64(&mut self, data: u64) -> Result<()>;
}

impl<W: Write> WriteExt for W {
    /// Write a [`u16`]
    fn write_u16(&mut self, data: u16) -> Result<()> {
        self.write_all(&data.to_le_bytes())
    }

    /// Write a [`u32`]
    fn write_u32(&mut self, data: u32) -> Result<()> {
        self.write_all(&data.to_le_bytes())
    }

    /// Write a [`u64`]
    fn write_u64(&mut self, data: u64) -> Result<()> {
        self.write_all(&data.to_le_bytes())
    }
}

use std::io::{Result, Write};
