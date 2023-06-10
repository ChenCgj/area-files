use std::net::{SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Handle;
use area_files_lib::Server;
use config::Config;

mod config;

static mut GLOBAL_CONFIG: Option<Config> = None;

pub fn run(config_path: &str) {
    match Config::load_config(config_path) {
        Ok(config) => unsafe { GLOBAL_CONFIG = Some(config); },
        Err(e) => {
            eprintln!("load configure fail: {}", e);
            return;
        }
    }

    let config = unsafe { GLOBAL_CONFIG.as_ref().unwrap() };

    let mut server: Server;
    match Server::new() {
        Ok(_server) => server = _server,
        Err(e) => {
            eprintln!("create server fail: {}", e);
            return;
        }
    }

    match server.add_tcp_server(config.get_host_ip(), config.get_host_port(), tcp_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add tcp server fail: {}", e);
            return;
        }
    }

    match server.add_tcp_server(config.get_client_listen_ip(), config.get_client_listen_port(), client_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add client tcp handler fail: {}", e);
            return;
        }
    }

    match server.add_udp_server(config.get_host_ip(), config.get_host_port(), udp_handler) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add udp server fail: {}", e);
            return;
        }
    }

    server.run();
}

pub fn tcp_handler(stream: TcpStream, addr: SocketAddr, handle: Handle) {
    println!("tcp: {} connect", addr);
    handle.spawn(async move {
        let mut stream = stream;
        match stream.write(&['h' as u8;32]).await {
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
                match udp_socket.send_to("hello world".as_bytes(), addr).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("reply error: {:?}", e)
                }
            }
            Err(e) => eprintln!("udp error: {:?}", e)
        }
    });
}

pub fn client_handler(stream: TcpStream, addr: SocketAddr, handle: Handle) {
    println!("client connect");
    handle.spawn(async move {
        let mut stream = stream;
        let mut buf = vec![];
        match stream.read(&mut buf).await {
            Ok(size) => {
                match stream.write_all("hello".as_bytes()).await {
                    Ok(_) => {},
                    Err(e) => eprintln!("couldn't response to client: {:?}", e)
                }
            }
            Err(e) => eprintln!("couldn't read the data from the client: {:?}", e)
        }
    });
}
