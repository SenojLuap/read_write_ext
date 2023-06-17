pub trait WriteExt {
    /// Write a [`Writable`]
    fn write_ext<W: Writable>(&mut self, data: &W) -> Result<()>;
}

impl<W: Write> WriteExt for W {
    /// Write a [`Writable`]
    fn write_ext<Wr: Writable>(&mut self, data: &Wr) -> Result<()> {
        data.write(self)
    }
}

use std::io::{Result, Write};

use crate::Writable;
