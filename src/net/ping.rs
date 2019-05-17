use pnet::packet::icmp::echo_request;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::Packet;
use pnet::transport::TransportSender;
use pnet::util;
use pnet_macros_support::types::*;
use rand::random;
use std::net::IpAddr;

pub fn send_echo(tx: &mut TransportSender, addr: IpAddr) {
    // Allocate enough space for a new packet
    let mut vec: Vec<u8> = vec![0; 16];

    // Use echo_request so we can set the identifier and sequence number
    let mut echo_packet = echo_request::MutableEchoRequestPacket::new(&mut vec[..]).unwrap();
    echo_packet.set_sequence_number(random::<u16>());
    echo_packet.set_identifier(random::<u16>());
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);

    let csum = icmp_checksum(&echo_packet);
    echo_packet.set_checksum(csum);

    tx.send_to(echo_packet, addr).unwrap();

    // let (thread_tx, thread_rx) = mpsc::channel();
    // let t = thread::spawn(move || {
    //     let mut rx = icmp_packet_iter(&mut rx);
    //     match rx.next() {
    //         Ok((_, addr)) => thread_tx.send(Some(addr)),
    //         Err(e) => panic!("error: {:?}", e),
    //     }
    // });

    // thread::sleep(std::time::Duration::new(5, 0));

    // match thread_rx.try_recv() {
    //     Ok(Some(addr)) => print!("ping: {}", addr),
    //     Ok(None) => println!("stupid error"),
    //     Err(mpsc::TryRecvError::Empty) => {
    //         drop(thread_rx);
    //         drop(t);
    //     },
    //     Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
    // }
}

fn icmp_checksum(packet: &echo_request::MutableEchoRequestPacket) -> u16be {
    util::checksum(packet.packet(), 1)
}
