#[macro_use] extern crate log;
extern crate env_logger;
extern crate kissshot;

use std::net::TcpStream;

use kissshot::message::Message;

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
    let msg = try!(client.reader.parse_raw_packet(packet));
    match msg {
        Message::KexInitMsg(kex_init) => {
            macro_rules! info_list {
                ($a:expr) => (
                    info!("{}: `{:?}`", stringify!($a), $a.0.iter().map(|s| String::from_utf8_lossy(s)).collect::<Vec<_>>());
                )
            }

            info_list!(kex_init.kex_algorithms);
            info_list!(kex_init.server_host_key_algorithms );
            info_list!(kex_init.encryption_algorithms_client_to_server );
            info_list!(kex_init.encryption_algorithms_server_to_client );
            info_list!(kex_init.mac_algorithms_client_to_server );
            info_list!(kex_init.mac_algorithms_server_to_client );
            info_list!(kex_init.compression_algorithms_client_to_server );
            info_list!(kex_init.compression_algorithms_server_to_client );
            info_list!(kex_init.languages_client_to_server );
            info_list!(kex_init.languages_server_to_client );
            info!("follows: {}", kex_init.first_kex_packet_follows);
            info!("reservec: {}", kex_init._reserved);
        }
    }


    // try!(client.close());

    Ok(())
}
