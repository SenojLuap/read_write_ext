pub trait ReadExt {
    /// Read a [`Readable`]
    fn read_ext<R: Readable>(&mut self) -> Result<R>;
}

impl<R: Read> ReadExt for R {
   /// Read a [`Readable`]
    fn read_ext<Re: Readable>(&mut self) -> Result<Re> {
        Re::read(self)
    }
}


use std::io::{Read, Result};

use crate::Readable;