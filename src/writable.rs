/// An inversion of the [`Write`]/[`WriteExt`](crate::write::WriteExt) pattern.
pub trait Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

impl<Wr> Writable for Vec<Wr>
where
    Wr: Writable,
{
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(&self.len())?;
        for value in self.iter() {
            writer.write_ext(value)?;
        }
        Ok(())
    }
}

impl Writable for usize {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let res = self
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Unsupported, "could not convert to usize"))?;
        writer.write_ext(res)
    }
}

impl Writable for u8 {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[*self; 1])
    }
}

impl Writable for u16 {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl Writable for u32 {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl Writable for u64 {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl Writable for str {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.as_bytes();
        writer.write_ext(&bytes.len())?;
        writer.write_all(bytes)
    }
}

use std::io::{Error, ErrorKind, Result, Write};

use crate::WriteExt;
