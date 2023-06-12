use std::net::{SocketAddr};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use config::Config;
use data::AreaFilesData;
use json;

use area_files_lib::{Server};

mod config;
mod data;
mod handler;

use handler::*;

static mut GLOBAL_CONFIG: Option<Config> = None;
static mut GLOBAL_DATA: Option<Arc<Mutex<AreaFilesData>>> = None;

pub fn get_config() -> &'static Config {
    unsafe { GLOBAL_CONFIG.as_ref().unwrap() }
}

pub fn get_data() -> &'static Arc<Mutex<AreaFilesData>> {
    unsafe { GLOBAL_DATA.as_ref().unwrap() }
}

pub fn run(config_path: &str) {
    match Config::load_config(config_path) {
        Ok(config) => unsafe { GLOBAL_CONFIG = Some(config); },
        Err(e) => {
            eprintln!("load configure fail: {}", e);
            return;
        }
    }

    unsafe {
        GLOBAL_DATA = Some(Arc::new(Mutex::new(AreaFilesData::new())));
    }

    let config = get_config();

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
        match stream.write(b"helloworld").await {
            Ok(_) => {}
            Err(e) => eprintln!("tcp error: {:?}", e)
        }
    });
}

pub fn udp_handler(buf: &[u8], addr: SocketAddr, handle: Handle) {
    let message = String::from_utf8_lossy(buf);
    println!("udp: {} send {}", addr, &message);
    let json_message;
    match json::parse(&message) {
        Ok(json_str) => json_message = json_str,
        Err(e) => {
            eprintln!("parse message error: {}", e);
            return;
        }
    }
    if !json_message.has_key("msg_type") {
        eprintln!("invalid message without \"msg_type\"");
        return;
    }
    match &json_message["msg_type"] {
        t if t == "query_area" => handle_query_area(&handle, addr, get_config()),
        t if t == "reply_area" => handle_reply_area(&handle, &message, get_data().clone()),
        _ => {
            eprintln!("unknown message type: {}", json_message)
        }
    }
}

pub fn client_handler(stream: TcpStream, _addr: SocketAddr, handle: Handle) {
    println!("client connect");
    handle.spawn(async move {
        let mut reader = BufReader::new(stream);
        loop {
            // let mut buf = Vec::from([0;4096]);
            let mut buf = String::new();
            buf.reserve(4096);
            match reader.read_line(&mut buf).await {
                Ok(0) => { println!("client disconnect"); break },
                Ok(size) => {
                    if size >= buf.capacity() {
                        match reader.get_mut().write_all("cmd too long".as_bytes()).await {
                            Ok(_) => {},
                            Err(e) => eprintln!("response fail: {}", e.to_string())
                        }
                        continue
                    }
                    handle_client_command(&mut reader, buf).await;
                }
                Err(e) => {
                    eprintln!("couldn't read the data from the client: {:?}", e);
                    break
                }
            }
        }
    });
}

async fn handle_client_command(reader: &mut BufReader<TcpStream>, buf: String) {
    let cmd = buf.trim().to_string();
    if cmd == "CMD update-file-info" {
        handle_update_file_info(get_config(), reader.get_mut()).await;
    } else if cmd == "CMD list my" {
        handle_list_my(get_data().clone(), reader.get_mut()).await;
    } else {
        match reader.get_mut().write_all(format!("unknown command: {}\n", cmd).as_bytes()).await {
            Ok(_) => {},
            Err(e) => eprintln!("response fail: {}", e.to_string())
        }
    }
}

