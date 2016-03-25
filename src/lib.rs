#[macro_use] extern crate log;

use std::str;
use std::io;
use std::io::prelude::*;

pub mod result;
pub mod util;

pub use result::{SshError, SshResult};

use util::ReadExt;

pub struct Reader<R: Read> {
    reader: R,
    buf: Vec<u8>,
}

impl<R: Read> Reader<R> {
    fn new(reader: R) -> Reader<R> {
        Reader {
            reader: reader,
            buf: Vec::new(),
        }
    }

    /// Pull new data from `self.reader` into `self.buf`.
    /// Returns the number of bytes read.
    fn read_to_buf(&mut self) -> io::Result<usize> {
        // TODO XXX inefficient; use buf's internal buffer
        let mut buf = [0u8; 1024];
        let len = try!(self.reader.read(&mut buf));
        // info!("read_to_buf: received `{:?}`", &buf[..len]);
        info!("read_to_buf: received `{:?}` (utf8: {})", &buf[..len], String::from_utf8_lossy(&buf[..len]));
        if len > 0 {
            self.buf.extend_from_slice(&buf);
        }
        Ok(len)
    }

    /// Read a string that ends with "\r\n" (CR LF).
    /// The line may be an identification string (b"SSH-2.0-[version string]\r\n").
    /// This may return empty line `Vec::new()`.
    fn read_line(&mut self) -> io::Result<Vec<u8>> {
        // `self.buf[..buf_pos]` doesn't have `\r\n`
        let mut buf_pos = 0;
        let mut ret_len = 0;
        'a: loop {
            let buflen = self.buf.len();
            if buflen > 0 {
                for i in buf_pos..(buflen - 1) {
                    if self.buf[i] == b'\r' && self.buf[i + 1] == b'\n' {
                        ret_len = i + 2;
                        break 'a;
                    }
                }

                buf_pos = buflen - 1;
            }
            try!(self.read_to_buf());
        }

        let (ret, new_buf) = {
            let (ret, new_buf) = self.buf.split_at(ret_len);
            let ret = ret.to_vec(); // XXX truncate self.buf?
            let new_buf = new_buf.to_vec();
            (ret, new_buf)
        };
        self.buf = new_buf;
        info!("read_line: received `{:?}` (utf8: {})", ret, String::from_utf8_lossy(&ret));
        return Ok(ret);
    }

    pub fn read_raw_packet(&mut self) -> SshResult<Vec<u8>> {
        let packet_len = try!(self.reader.read_be_u32());
        info!("read_packet: packet_len {}", packet_len);
        let padding_len = try!(self.reader.read_u8()) as u32;
        info!("read_packet: padding_len {}", padding_len);
        if padding_len < 4 || packet_len <= padding_len {
            // TODO return SshError
            panic!("padding is wrong omg packet_len {} padding_len {}", packet_len, padding_len);
        }
        let payload_len = packet_len - padding_len - 1;
        let payload = try!(self.reader.read_exact_to_vec(payload_len as usize));
        let _padding = try!(self.reader.read_exact_to_vec(padding_len as usize));
        // TODO mac
        Ok(payload)
    }
}

pub struct Writer<W: Write> {
    writer: W,
}

impl<W: Write> Writer<W> {
    fn new(writer: W) -> Writer<W> {
        Writer {
            writer: writer,
        }
    }

    /// Proxy method of `Write::write_all()` with logging.
    fn write_raw(&mut self, buf: &[u8]) -> io::Result<()> {
        info!("write_raw buf: `{:?}`", buf);
        try!(self.writer.write_all(buf));
        Ok(())
    }
}

pub struct Client<R: Read, W: Write> {
    pub reader: Reader<R>,
    pub writer: Writer<W>,
}

impl<R: Read, W: Write> Client<R, W> {
    pub fn new(reader: R, writer: W) -> Client<R, W> {
        let reader = Reader::new(reader);
        let writer = Writer::new(writer);

        Client {
            reader: reader,
            writer: writer,
        }
    }

    pub fn connect(&mut self) -> SshResult<()> {
        let id = b"SSH-2.0-kissshot Acerola-Orion Heart-Under-Blade\r\n";
        try!(self.writer.write_raw(id));

        // receive server version
        loop {
            let line = try!(self.reader.read_line());
            if line.len() > 3 && &line[..3] == b"SSH" {
                let line = String::from_utf8_lossy(&line);
                debug!("connect: server version `{}`", line);
                break;
            }
        }

        Ok(())
    }
}
