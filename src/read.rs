pub trait ReadExt {
    /// Read a [`u16`]
    fn read_u16(&mut self) -> Result<u16>;

    /// Read a [`u32`]
    fn read_u32(&mut self) -> Result<u32>;

    /// Read a [`u64`]
    fn read_u64(&mut self) -> Result<u64>;
}

impl<R: Read> ReadExt for R {
    /// Read a [`u16`]
    fn read_u16(&mut self) -> Result<u16> {
        let mut buffer = [0_u8; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Read a [`u32`]
    fn read_u32(&mut self) -> Result<u32> {
        let mut buffer = [0_u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Read a [`u64`]
    fn read_u64(&mut self) -> Result<u64> {
        let mut buffer = [0_u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}


use std::io::{Read, Result};