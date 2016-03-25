// XXX copied from suruga

use std::io::prelude::*;
use std::io;
use std::mem;

// read/write system endian integers.
macro_rules! read_write_prim {
    ($read_name:ident, $write_name:ident, $t:ty, $len:expr) => (
        #[inline(always)]
        fn $read_name<R: ?Sized + ReadExt>(mut reader: &mut R) -> io::Result<$t> {
            let mut buf = [0u8; $len];
            try!(reader.read_exact(&mut buf));
            let value: $t = unsafe { mem::transmute(buf) };
            Ok(value)
        }
        #[inline(always)]
        fn $write_name<R: ?Sized + Write>(mut writer: &mut R, value: $t) -> io::Result<()> {
            let buf: [u8; $len] = unsafe { mem::transmute(value) };
            try!(writer.write_all(&buf));
            Ok(())
        }
    )
}

read_write_prim!(read_u8, write_u8, u8, 1);
read_write_prim!(read_u16, write_u16, u16, 2);
read_write_prim!(read_u32, write_u32, u32, 4);
read_write_prim!(read_u64, write_u64, u64, 8);


pub trait ReadExt: Read {
    #[inline(always)]
    fn read_exact_to_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut vec = vec![0u8; len];
        try!(self.read_exact(&mut vec));
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

