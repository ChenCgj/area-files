use std::error::Error;
use std::io::ErrorKind;
use std::net::{SocketAddr};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::Mutex;
use config::Config;
use data::AreaFilesData;
use json;
use json::JsonValue;
use json::short::Short;

use area_files_lib::{Server, User};
use area_files_lib::file_mgr::{gen_info_for, get_all_info};
use area_files_lib::util::construct_tcp_packet;

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

pub fn prepare_dir(config: &Config, data: &Arc<Mutex<AreaFilesData>>) -> Result<(), String> {
    let share_dir = config.get_shared_path();
    let download_dir = config.get_download_path();
    let info_dir;
    match std::path::Path::new(share_dir).join(".info_dir").to_str() {
        Some(p) => info_dir = p.to_string(),
        None => return Err("get info dir path fail".to_string())
    }
    if let Err(e) = std::fs::create_dir(share_dir) {
        if e.kind() != ErrorKind::AlreadyExists {
            return Err(format!("create shared dir fail: {}", e))
        }
    }
    if let Err(e) = std::fs::create_dir(download_dir) {
        if e.kind() != ErrorKind::AlreadyExists {
            return Err(format!("create download dir fail: {}", e))
        }
    }
    if let Err(e) = std::fs::create_dir(info_dir) {
        if e.kind() != ErrorKind::AlreadyExists {
            return Err(format!("create info dir fail: {}", e))
        }
    }
    if let Err(e) = gen_info_for(
        share_dir,
        true,
        &User::UserLAN {
            host_name: config.get_localhost_name().to_string(),
            ip: config.get_host_ip().to_string()
        }) {

        return Err(format!("gen info fail: {}", e))
    }
    match data.try_lock() {
        Ok(mut d) => {
            let info;
            match get_all_info(share_dir) {
                Ok(i) => info = i,
                Err(e) => return Err(format!("get my file info fail: {}", e))
            }
            d.update_my_files(&info);
        }
        Err(e) => {
            return Err(format!("get the data lock fail: {}", e));
        }
    }
    Ok(())
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

    if let Err(e) = prepare_dir(config, get_data()) {
        eprintln!("init fail: {}", e);
        return;
    }

    // print info
    {
        println!("host ip: {}", config.get_host_ip());
        println!("server (ip:port): {}:{}", config.get_server_ip(), config.get_server_port());
        println!("broadcast ip: {}", config.get_broadcast_ip());
        println!("listen port: {}", config.get_host_port());
        println!("client listen (ip:port): {}:{}", config.get_client_listen_ip(), config.get_client_listen_port())
    }

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
        let mut json_str_size_buf = Vec::from([0u8; 16]);
        if let Err(e) = stream.read_exact(&mut json_str_size_buf).await {
            eprintln!("read from the stream fail: {}", e);
            return;
        }
        let mut json_reply = JsonValue::new_object();
        json_reply["msg_type"] = JsonValue::from("error");
        json_reply["statu"] = JsonValue::from(-1);
        json_reply["message"] = JsonValue::from("invalid message");

        let mut json_str_size;
        match String::from_utf8_lossy(&json_str_size_buf).parse::<u32>() {
            Ok(s) => json_str_size = s,
            Err(e) => {
                match stream.write_all(&construct_tcp_packet(&json_reply)).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("response fail: {}", e)
                }
                return;
            }
        }

        let mut json_buf = Vec::new();
        // json_buf.reserve(json_str_size as usize);
        json_buf.resize(json_str_size as usize, 0);

        if let Err(e) = stream.read_exact(&mut json_buf).await {
            match stream.write_all(&construct_tcp_packet(&json_reply)).await {
                Ok(_) => {}
                Err(e) => eprintln!("response fail: {}", e)
            }
            return;
        }

        let json_str = String::from_utf8_lossy(&json_buf);
        let mut json_data = JsonValue::Null;
        match json::parse(&json_str) {
            Ok(d) => json_data = d,
            Err(_) => {
                match stream.write_all(&construct_tcp_packet(&json_reply)).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("response fail: {}", e)
                }
            }
        }

        if !json_data.has_key("msg_type") {
            match stream.write_all(&construct_tcp_packet(&json_reply)).await {
                Ok(_) => {}
                Err(e) => eprintln!("response fail: {}", e)
            }
        }

        if json_data["msg_type"].as_str().unwrap() == "request_area" {
            handle_request_area(&json_data, &mut stream, get_config(), get_data().clone()).await;
        } else {
            json_reply["message"] = JsonValue::from(format!("unknown message type: {}", json_data["msg_type"]));
            match stream.write_all(&construct_tcp_packet(&json_reply)).await {
                Ok(_) => {}
                Err(e) => eprintln!("response fail: {}", e)
            }
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
        t if t == "reply_area" => handle_reply_area(&handle, addr, get_config(), &message, get_data().clone()),
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
    } else if cmd == "CMD list hosts" {
        handle_list_other(get_data().clone(), reader.get_mut()).await;
    } else if cmd.len() > 12 && &cmd[..12] == "CMD download" {
        handle_download(&cmd, get_data().clone(), reader.get_mut(), &get_config()).await;
    } else if cmd.len() == 0 {
        // nothing here
    } else {
        match reader.get_mut().write_all(format!("unknown command: {}", cmd).as_bytes()).await {
            Ok(_) => {},
            Err(e) => eprintln!("response fail: {}", e.to_string())
        }
    }
}

