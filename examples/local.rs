#[macro_use] extern crate log;
extern crate env_logger;
extern crate kissshot;

use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    env_logger::init().unwrap();
    test().expect("test failed");
}

fn test() -> kissshot::SshResult<()> {
    let stream_in = try!(TcpStream::connect("localhost:22"));
    let stream_out = try!(stream_in.try_clone());
    let mut client = kissshot::Client::new(stream_in, stream_out);
    try!(client.connect());

    let packet = try!(client.reader.read_raw_packet());
    println!("packet: {:?} ({})", packet, String::from_utf8_lossy(&packet));

    // try!(client.close());

    Ok(())
}
