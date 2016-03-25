use result::{SshError, SshResult};
use util::{ReadExt, WriteExt};

/// A trait for items that can be serialized at SSH stream.
/// Follows RFC 4251 "Data Type Representations Used in the SSH Protocols"
pub trait SshItem {
    /// Write an item into SSH stream.
    fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> SshResult<()>;
    /// Read an item from SSH stream.
    fn ssh_read<R: ReadExt>(reader: &mut R) -> SshResult<Self> where Self: Sized;
    /// Returns the length of serialized bytes.
    fn ssh_size(&self) -> u64; // TODO u32?
}

// copied from suruga TODO

macro_rules! num_size {
    (u8) => (1);
    (u16) => (2);
    (u32) => (4);
    (u64) => (8);
}

macro_rules! try_write_num {
    (u8, $writer:expr, $e:expr) => ({
        try!($writer.write_u8($e as u8));
    });
    (u16, $writer:expr, $e:expr) => ({
        try!($writer.write_be_u16($e as u16));
    });
    (u32, $writer:expr, $e:expr) => ({
        try!($writer.write_be_u32($e as u32));
    });
    (u64, $writer:expr, $e:expr) => ({
        try!($writer.write_be_u64($e as u64));
    });
}

macro_rules! try_read_num {
    (u8, $reader:expr) => ({
        try!($reader.read_u8())
    });
    (u16, $reader:expr) => ({
        try!($reader.read_be_u16())
    });
    (u32, $reader:expr) => ({
        try!($reader.read_be_u32())
    });
    (u64, $reader:expr) => ({
        try!($reader.read_be_u64())
    });
}

// implementation of `SshItem` for primitive integer types like `u8`
macro_rules! ssh_primitive {
    ($t:ident) => (
        impl SshItem for $t {
            fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
                try_write_num!($t, writer, *self);
                Ok(())
            }

            fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<$t> {
                let u = try_read_num!($t, reader);
                Ok(u)
            }

            fn ssh_size(&self) -> u64 { num_size!($t) }
        }
    )
}

ssh_primitive!(u8);
ssh_primitive!(u16);
ssh_primitive!(u32);
ssh_primitive!(u64);

impl SshItem for bool {
    fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
        try!(writer.write_all(&[*self as u8]));
        Ok(())
    }

    fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<bool> {
        let u = try!(reader.read_u8());

        if u != 0 && u != 1 {
            error!("bool::ssh_read: found non-bool value `{}`", u);
            // TODO return error
        }

        Ok(u != 0)
    }

    fn ssh_size(&self) -> u64 { 1 }
}

macro_rules! ssh_bytes {
    ($len:expr) => (
        impl SshItem for [u8; $len] {
            fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
                try!(writer.write_all(&self[..]));
                Ok(())
            }

            fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<[u8; $len]> {
                let mut buf = [0u8; $len];
                try!(reader.read_exact(&mut buf));
                Ok(buf)
            }

            fn ssh_size(&self) -> u64 {
                let len: usize = $len;
                len as u64
            }
        }
    )
}

ssh_bytes!(16);

pub struct NameList(pub Vec<Vec<u8>>);

impl SshItem for NameList {
    fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
        unimplemented!();
    }

    fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<NameList> {
        let size = try!(u32::ssh_read(reader)) as usize;
        let mut data = vec![0u8; size];
        try!(reader.read_exact(&mut data));
        let data: Vec<_> = data.split(|c| *c == b',').map(|s| s.to_vec()).collect();
        Ok(NameList(data))
    }

    fn ssh_size(&self) -> u64 { unimplemented!(); }
}

macro_rules! ssh_message_body {
    (
        struct $name:ident {
            $(
                $field:ident: $t:ty,
            )+
        }
    ) => (
        pub struct $name {
            $(
                pub $field: $t,
            )+
        }

        impl SshItem for $name {
            fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
                unimplemented!();
            }

            fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<$name> {
                $(
                    let $field: $t = try!(SshItem::ssh_read(reader));
                )*
                let ret: $name = $name {
                    $(
                        $field: $field,
                    )+
                };
                Ok(ret)
            }

            fn ssh_size(&self) -> u64 { unimplemented!(); }
        }
    )
}

ssh_message_body!(
    struct KexInit {
        cookie: [u8; 16],
        kex_algorithms: NameList,
        server_host_key_algorithms: NameList,
        encryption_algorithms_client_to_server: NameList,
        encryption_algorithms_server_to_client: NameList,
        mac_algorithms_client_to_server: NameList,
        mac_algorithms_server_to_client: NameList,
        compression_algorithms_client_to_server: NameList,
        compression_algorithms_server_to_client: NameList,
        languages_client_to_server: NameList,
        languages_server_to_client: NameList,
        first_kex_packet_follows: bool,
        _reserved: u32,
    }
);

macro_rules! ssh_messages {
    (
        enum $name:ident {
            $(
                $field:ident($field_ty:ty) = $number:expr,
            )+
        }
    ) => (
        pub enum $name {
            $(
                $field($field_ty),
            )+
        }

        impl SshItem for $name {
            fn ssh_write<W: WriteExt>(&self, writer: &mut W) -> ::result::SshResult<()> {
                unimplemented!();
            }

            fn ssh_read<R: ReadExt>(reader: &mut R) -> ::result::SshResult<$name> {
                let msg_num: u8 = try!(SshItem::ssh_read(reader));
                $(
                    if msg_num == $number {
                        let body: $field_ty = try!(SshItem::ssh_read(reader));
                        return Ok($name::$field(body));
                    }
                )+

                // TODO unknown msg
                unimplemented!();
            }

            fn ssh_size(&self) -> u64 { unimplemented!(); }
        }
    )
}

ssh_messages!(
    enum Message {
        KexInitMsg(KexInit) = 20,
    }
);
