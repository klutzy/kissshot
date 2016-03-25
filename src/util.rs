// XXX copied from suruga

use std::io::prelude::*;
use std::io;

pub trait ReadExt: Read {
    /// Fill buf completely or return `Err`.
    /// NOTE: the default implementation returns `Err(io::ErrorKind::Other)` if EOF is found.
    /// this may be not desired if the source is non-blocking.
    #[inline(always)]
    fn fill_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let len = buf.len();
        let mut pos = 0;
        while pos < len {
            let num_bytes = try!(self.read(&mut buf[pos..]));
            if num_bytes == 0 {
                return Err(io::Error::new(io::ErrorKind::Other, SurugaError {
                    desc: "EOF during `fill_exact`",
                    cause: None
                }));
            }
            pos += num_bytes;
        }
        Ok(())
    }

    #[inline(always)]
    fn read_exact(&mut self, len: usize) -> io::Result<Vec<u8>> {
        // FIXME this can be more efficient using unsafe methods
        let mut vec = vec![0u8; len];
        try!(self.fill_exact(&mut vec));
        Ok(vec)
    }

    #[inline(always)]
    fn read_u8(&mut self) -> io::Result<u8> {
        read_u8(self)
    }
    #[inline(always)]
    fn read_be_u16(&mut self) -> io::Result<u16> {
        let value: u16 = try!(read_u16(self));
        Ok(value.to_be())
    }
    #[inline(always)]
    fn read_le_u16(&mut self) -> io::Result<u16> {
        let value: u16 = try!(read_u16(self));
        Ok(value.to_le())
    }
    #[inline(always)]
    fn read_be_u32(&mut self) -> io::Result<u32> {
        let value: u32 = try!(read_u32(self));
        Ok(value.to_be())
    }
    #[inline(always)]
    fn read_le_u32(&mut self) -> io::Result<u32> {
        let value: u32 = try!(read_u32(self));
        Ok(value.to_le())
    }
    #[inline(always)]
    fn read_be_u64(&mut self) -> io::Result<u64> {
        let value: u64 = try!(read_u64(self));
        Ok(value.to_be())
    }
    #[inline(always)]
    fn read_le_u64(&mut self) -> io::Result<u64> {
        let value: u64 = try!(read_u64(self));
        Ok(value.to_le())
    }
}

impl<R: Read> ReadExt for R {}

pub trait WriteExt: Write {
    #[inline(always)]
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        write_u8(self, value)
    }

    #[inline(always)]
    fn write_be_u16(&mut self, value: u16) -> io::Result<()> {
        write_u16(self, value.to_be())
    }
    #[inline(always)]
    fn write_le_u16(&mut self, value: u16) -> io::Result<()> {
        write_u16(self, value.to_le())
    }

    #[inline(always)]
    fn write_be_u32(&mut self, value: u32) -> io::Result<()> {
        write_u32(self, value.to_be())
    }
    #[inline(always)]
    fn write_le_u32(&mut self, value: u32) -> io::Result<()> {
        write_u32(self, value.to_le())
    }

    #[inline(always)]
    fn write_be_u64(&mut self, value: u64) -> io::Result<()> {
        write_u64(self, value.to_be())
    }
    #[inline(always)]
    fn write_le_u64(&mut self, value: u64) -> io::Result<()> {
        write_u64(self, value.to_le())
    }
}

