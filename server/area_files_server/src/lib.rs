use std::net::{SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Handle;
use area_files_lib::Server;
use config::Config;

mod config;

pub fn tcp_handler(stream: TcpStream, addr: SocketAddr, handle: Handle) {
    println!("tcp: {} connect", addr);
    handle.spawn(async move {
        let mut stream = stream;
        match stream.write_all("hello".as_bytes()).await {
            Ok(_) => {}
            Err(e) => eprintln!("tcp error: {:?}", e)
        }
    });
}

pub fn udp_handler(buf: &[u8], addr: SocketAddr, handle: Handle) {
    println!("udp: {} send {}", addr, String::from_utf8_lossy(buf));
    handle.spawn(async move {
        match UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await {
            Ok(udp_socket) => {
                match udp_socket.send_to("hello".as_bytes(), addr).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("reply error: {:?}", e)
                }
            }
            Err(e) => eprintln!("udp error: {:?}", e)
        }
    });
}

pub fn run(config_path: &str) {
    let config: Config;
    match Config::load_config(config_path) {
        Ok(_config) => config = _config,
        Err(e) => {
            eprintln!("load configure fail: {}", e);
            return;
        }
    }

    let mut server: Server;
    match Server::new() {
        Ok(_server) => server = _server,
        Err(e) => {
            eprintln!("create server fail: {}", e);
            return;
        }
    }

    match server.add_tcp_server(config.get_server_ip(), config.get_server_port(), tcp_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add tcp server fail: {}", e);
            return;
        }
    }

    match server.add_udp_server(config.get_server_ip(), config.get_server_port(), udp_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add udp server fail: {}", e);
            return;
        }
    }

    server.run();
}

