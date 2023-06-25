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
        let mut buffer = vec![0_u8; len];
        reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "could not convert from utf-8"))
    }
}

impl<Re> Readable for Vec<Re>
where
    Re: Readable,
{
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let len = reader.read_ext()?;
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(reader.read_ext()?);
        }
        Ok(res)
    }
}

impl<RKey, RValue> Readable for HashMap<RKey, RValue>
where
    RKey: Readable + Eq + Hash,
    RValue: Readable,
{
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let len = reader.read_ext()?;
        let mut res = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = reader.read_ext()?;
            let value = reader.read_ext()?;
            res.insert(key, value);
        }
        Ok(res)
    }
}

use std::{
    collections::HashMap,
    hash::Hash,
    io::{Error, ErrorKind, Read, Result},
};

use crate::ReadExt;
