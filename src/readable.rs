pub trait Readable
where
    Self: Sized,
{
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
}

impl Readable for u8 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0_u8; 1];
        reader.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl Readable for u16 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0_u8; 2];
        reader.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

impl Readable for u32 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0_u8; 4];
        reader.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl Readable for u64 {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0_u8; 8];
        reader.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

impl Readable for usize {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let res: u64 = reader.read_ext()?;
        res.try_into()
            .map_err(|_| Error::new(ErrorKind::Unsupported, "could not convert to u64"))
    }
}

impl Readable for String {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let len: usize = reader.read_ext()?;
        let mut buffer = Vec::with_capacity(len);
        reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "could not convert from utf-8"))
    }
}

use std::io::{Error, ErrorKind, Read, Result};

use crate::ReadExt;
