use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use json::JsonValue;
use pnet::datalink;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub fn get_local_ip() -> Vec<String> {
    let mut ret = vec![];
    for ifs in datalink::interfaces() {
        for ip in ifs.ips.iter() {
            match ip.ip() {
                IpAddr::V4(ip4) => {
                    if ip4.is_loopback() || ip4 == Ipv4Addr::new(0, 0, 0, 0) || ip4 == Ipv4Addr::LOCALHOST {
                        continue
                    }
                }
                IpAddr::V6(ip6) => {
                    if ip6.is_loopback() || ip6 == Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0) || ip6 == Ipv6Addr::LOCALHOST {
                        continue
                    }
                }
            }
            ret.push(ip.ip().to_string())
        }
    }
    ret
}

pub fn construct_tcp_packet(json_v: &JsonValue) -> Vec<u8> {
    let json_str = json_v.to_string();
    let json_size = format!("{:016}", json_str.len());
    let mut ret = Vec::from(json_size);
    ret.append(&mut Vec::from(json_str));
    ret
}

pub async fn send_tcp_packet(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    match stream.write_all(buf).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("send tcp packet fail: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let ip = get_local_ip();
        print!("{:?}", ip)
    }
}