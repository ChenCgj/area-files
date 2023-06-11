use std::borrow::BorrowMut;
use std::fmt::format;
use std::net::{SocketAddr};
use std::ops::Deref;
use std::sync::Arc;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use config::Config;
use data::AreaFilesData;
use json;

use area_files_lib::{Server, protocol, file_mgr};
use area_files_lib::User::UserLAN;

mod config;
mod data;

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
        match stream.write(&['h' as u8;32]).await {
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
        t if t == "query_area" => {
            handle.spawn(async move {
                let mut addr = addr;
                match reply_get_file_information(&mut addr).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("send reply error: {:?}", e)
                }
            });
        }
        t if t == "reply_area" => {
            match serde_json5::from_str(&message) {
                Ok(info) =>  {
                    handle.spawn(async move {
                        match update_info(&info).await {
                            Ok(_) => {}
                            Err(e) => eprintln!("update info fail: {}", e)
                        }
                    });
                }
                Err(e) => eprintln!("parse info fail: {}", e.to_string())
            }
        }
        _ => {
            eprintln!("unknown message type: {}", json_message)
        }
    }
}

pub fn client_handler(stream: TcpStream, _addr: SocketAddr, handle: Handle) {
    println!("client connect");
    handle.spawn(async move {
        let mut stream = stream;
        loop {
            let mut buf = Vec::from([0;4096]);
            match stream.read(&mut buf).await {
                Ok(size) => {
                    if size >= buf.capacity() {
                        match stream.write_all("cmd too long".as_bytes()).await {
                            Ok(_) => {},
                            Err(e) => eprintln!("response fail: {}", e.to_string())
                        }
                        continue
                    }
                    match String::from_utf8(buf[..size].to_vec()) {
                        Ok(mut cmd) => {
                            cmd = cmd.trim().to_string();
                            if cmd == "CMD update-file-info" {
                                match send_get_file_information().await {
                                    Ok(_) => {},
                                    Err(e) => {
                                        match stream.write_all(format!("couldn't response to client: {:?}", e).as_bytes()).await {
                                            Ok(_) => {},
                                            Err(e) => eprintln!("response fail: {}", e.to_string())
                                        }
                                    }
                                }
                            } else if cmd == "CMD list my" {
                                let d = serde_json5::to_string(&get_data().lock().await.my_files).unwrap();
                                match stream.write_all(d.as_bytes()).await {
                                    Ok(_) => {},
                                    Err(e) => eprintln!("response fail: {}", e.to_string())
                                }
                            } else {
                                match stream.write_all(format!("unknown command: {}\n", cmd).as_bytes()).await {
                                    Ok(_) => {},
                                    Err(e) => eprintln!("response fail: {}", e.to_string())
                                }
                            }
                        }
                        Err(e) => {
                            match stream.write_all(format!("invalid command: {}", e.to_string()).as_bytes()).await {
                                Ok(_) => {},
                                Err(e) => eprintln!("response fail: {}", e.to_string())
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("couldn't read the data from the client: {:?}", e);
                    break
                }
            }
        }
    });
}

async fn send_get_file_information() -> io::Result<()> {
    let config = get_config();
    let udp_socket = UdpSocket::bind(&format!("{}:{}", config.get_host_ip(), 0)).await?;
    udp_socket.set_broadcast(true)?;
    let ask: String;
    match protocol::MsgQueryArea::generate_json(0) {
        Ok(_a) => ask = _a,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("couldn't generate the request: {}", e.to_string())))
    }

    // need to check the ip valid when load config
    let addr = config.get_broadcast_ip().parse().unwrap();
    let addr = SocketAddr::new(addr, config.get_host_port());
    Server::udp_send(&udp_socket, &addr, ask.as_bytes()).await
}

async fn reply_get_file_information(addr: &mut SocketAddr) -> io::Result<()> {
    let config = get_config();
    let info_list = file_mgr::get_all_info(config.get_shared_path())?;
    let user = UserLAN {
        // rename the hostname...
        host_name: "localhost".to_string(),
        ip: config.get_host_ip().to_string()
    };
    let reply;
    match protocol::MsgReplyArea::generate_json(0, 0, &info_list, &user) {
        Ok(json_str) => reply = json_str,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("get file info fail: {}", e.to_string())))
    }
    let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
    addr.set_port(config.get_host_port());
    Server::udp_send(&udp_socket, addr, reply.as_bytes()).await
}

async fn update_info(msg: &protocol::MsgReplyArea) -> Result<(), String> {
    let mut data = get_data().lock().await;
    let user = &msg.user;
    data.update_other_files(user, &msg.info);
    Ok(())
}
