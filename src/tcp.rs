use std::net::{ToSocketAddrs, SocketAddr};

use mio::net::{TcpStream, TcpListener};

use crate::{header::{LineType, LogTag}, log::Log};
pub struct Tcp{}

impl Tcp {
    pub fn bind(addr:&str) -> Option<TcpListener> {
        let addr = addr.parse().unwrap();
        match TcpListener::bind(addr) {
            Ok(ret) => {
                return Some(ret);
            }

            Err(err) => {
                Log::error(format!("bind fail.take a look at the errors or try google with <hyper-v port netstat>|{}|{}",addr,err));
            }
        }
        None
    }

    pub fn dns_lookup(host:&str)  -> Option<SocketAddr> {
        match host.to_socket_addrs() {
            Ok(mut it) => { return it.next() }

            Err(err) => {
                Log::add(format!("dns lookup fail|{}|{}",host,err), LineType::Spider, &LogTag::Unexpected);
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