/// An inversion of the [`Write`]/[`WriteExt`](crate::write::WriteExt) pattern.
pub trait Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

impl Writable for bool {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[if *self { 1_u8 } else { 0_u8 }; 1])
    }
}

impl Writable for usize {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let res: u64 = (*self)
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Unsupported, "could not convert to u64"))?;
        writer.write_ext(&res)
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

impl Writable for String {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.as_bytes();
        writer.write_ext(&bytes.len())?;
        writer.write_all(bytes)
    }
}

impl Writable for &str {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.as_bytes();
        writer.write_ext(&bytes.len())?;
        writer.write_all(bytes)
    }
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

impl<WKey, WValue> Writable for HashMap<WKey, WValue>
where
    WKey: Writable + Eq + Hash,
    WValue: Writable,
{
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(&self.len())?;
        for (key, value) in self.iter() {
            writer.write_ext(key)?;
            writer.write_ext(value)?;
        }
        Ok(())
    }
}

impl<Wr> Writable for Option<Wr>
    where Wr: Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            Some(ref contents) => {
                writer.write_ext(&true)?;
                writer.write_ext(contents)
            }
            None => writer.write_ext(&false)
        }
    }
}

impl<Wr> Writable for Box<Wr>
    where Wr: Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(self.as_ref())
    }
}

impl <Wr> Writable for Box<[Wr]>
    where Wr: Writable {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(&self.as_ref().len())?;
        for elem in self.as_ref().iter() {
            writer.write_ext(elem)?;
        }
        Ok(())
    }
}

use std::{
    collections::HashMap,
    hash::Hash,
    io::{Error, ErrorKind, Result, Write},
};

use crate::WriteExt;
