pub mod ping;

use super::server::Server;

use std::time::{Duration};
use std::net::{IpAddr};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::BTreeMap;
use std::thread;
use pnet::transport::{icmp_packet_iter};
use pnet::transport::transport_channel;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::{Ipv4};
use pnet::transport::{TransportSender, TransportReceiver};
use pnet::packet::ip::IpNextHeaderProtocols;
use std::net::ToSocketAddrs;

enum PingResult {
    TimeOut(IpAddr),
    Receive{addr: IpAddr},
}

pub struct Pinger {
    addrs: BTreeMap<IpAddr, bool>,
    thread_tx: Sender<PingResult>,
    thread_rx: Receiver<PingResult>,
    rx: Arc<Mutex<TransportReceiver>>,
    tx: TransportSender,
}

impl Pinger {
    pub fn new() -> Pinger {
        let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
        let (tx, rx) = match transport_channel(4096, protocol) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => panic!("error: {}", e),
        };

        let (thread_tx, thread_rx) = mpsc::channel();

        Pinger {
            addrs: BTreeMap::new(),
            thread_tx: thread_tx,
            thread_rx: thread_rx,
            rx: Arc::new(Mutex::new(rx)),
            tx: tx,
        }
    }

    pub fn add_server(&mut self, server: &Server) {
        let host = resolve_host(&server.host);
        self.addrs.insert(host, false);
    }

    pub fn send_icmp(&mut self) {
        self.start_icmp_receiver();

        for (addr, reply) in self.addrs.iter_mut() {
            ping::send_echo(&mut self.tx, *addr);
            *reply = false;
        }

        loop {
            match self.thread_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(result) => {
                    match result {
                        PingResult::Receive{addr} => {
                            if let Some(reply) = self.addrs.get_mut(&addr) {
                                *reply = true;
                            }
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    break
                }
            }
        }

        for (addr, seen) in self.addrs.iter() {
            println!("IP: {} Reply: {}", addr, seen);
        }
    }

    fn start_icmp_receiver(&self) {
        let rx = self.rx.clone();
        let thread_tx = self.thread_tx.clone();

        thread::spawn(move || {
            let mut receiver = rx.lock().unwrap();
            let mut iter = icmp_packet_iter(&mut receiver);
            loop {
                match iter.next() {
                    Ok((_, addr)) => {
                        match thread_tx.send(PingResult::Receive{addr: addr}) {
                            Ok(_) => {},
                            Err(e) => panic!("error: {}", e),
                        }
                    },
                    Err(e) => panic!("error: {:?}", e),
                }
            }
        });
    }   
}

pub fn resolve_host(host: &str) -> IpAddr {
    let sock_addr = format!("{}:0", host);
    let addrs = sock_addr.to_socket_addrs()
        .map(|iter| 
        iter.map(|socket_address| socket_address.ip())
            .collect::<Vec<_>>()
        )
        .unwrap();
        
    addrs[0]
}
