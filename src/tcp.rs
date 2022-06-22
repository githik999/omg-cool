use std::net::{ToSocketAddrs, SocketAddr};

use mio::net::TcpStream;

use crate::{header::{LineType, LogTag}, log::Log};
pub struct Tcp{}

impl Tcp {
    pub fn dns_lookup(host:&str)  -> Option<SocketAddr> {
        match host.to_socket_addrs() {
            Ok(mut it) => { return it.next() }

            Err(e) => {
                Log::add(format!("dns lookup fail|{}|{}",host,e), LineType::Spider, &LogTag::Unexpected);
            }
        }
        None
    }

    pub fn connect(host:&str) -> Option<TcpStream> {
        match Tcp::dns_lookup(host) {
            Some(addr) => {
                match TcpStream::connect(addr) {
                    Ok(s) => { return Some(s) }
                    
                    Err(e) => {
                        Log::add(format!("connect fail|{}|{}",addr,e), LineType::Spider, &LogTag::Unexpected);
                    }
                }
            }
            _ => {}
        }
        None
    }
}