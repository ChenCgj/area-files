use std::fs;
use std::fs::read_to_string;
use std::io::{Read, Seek, Write};
use tokio::io;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::runtime::Handle;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use json::JsonValue;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};

use area_files_lib::{Server, user, file_mgr, protocol};
use area_files_lib::file_mgr::{FileInfo, get_info_from_file};
use area_files_lib::token_mgr;
use area_files_lib::token_mgr::Token;
use area_files_lib::util::{construct_tcp_packet, send_tcp_packet};
use crate::{AreaFilesData, config, Config};

async fn send_get_file_information(config: &config::Config) -> io::Result<()> {
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

async fn reply_get_file_information(config: &config::Config, addr: &mut SocketAddr) -> io::Result<()> {
    let info_list = file_mgr::get_all_info(config.get_shared_path())?;
    let user = user::User::UserLAN {
        // rename the hostname...
        host_name: config.get_localhost_name().to_string(),
        ip: config.get_host_ip().to_string()
    };
    let reply;
    match protocol::MsgReplyArea::generate_json(0, 0, &info_list, &user) {
        Ok(json_str) => reply = json_str,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("get file info fail: {}", e.to_string())))
    }
    let udp_socket = UdpSocket::bind(format!("{}:{}", config.get_host_ip(), 0)).await?;
    addr.set_port(config.get_host_port());
    Server::udp_send(&udp_socket, addr, reply.as_bytes()).await
}

async fn update_info(msg: &protocol::MsgReplyArea, data: Arc<Mutex<AreaFilesData>>) -> Result<(), String> {
    let mut data = data.lock().await;
    let user = &msg.user;
    data.update_other_files(user, &msg.info);
    Ok(())
}

async fn send_file(info: &FileInfo, stream: &TcpStream, config: &Config) {

}

// udp handler
pub fn handle_query_area(handle: &Handle, addr: SocketAddr, config: &'static Config) {
    // ignore the query from the local host
    if addr.ip() == config.get_host_ip().parse::<IpAddr>().unwrap() {
        return;
    }
    handle.spawn(async move {
        let mut addr = addr;
        match reply_get_file_information(config, &mut addr).await {
            Ok(_) => {}
            Err(e) => eprintln!("send reply error: {:?}", e)
        }
    });
}

pub fn handle_reply_area(handle: &Handle, addr: SocketAddr, config: &Config, message: &str, data: Arc<Mutex<AreaFilesData>>) {
    // ignore the broadcast reply from the local host
    if addr.ip() == config.get_host_ip().parse::<IpAddr>().unwrap() {
        return;
    }
    match serde_json5::from_str(&message) {
        Ok(info) =>  {
            handle.spawn(async move {
                match update_info(&info, data).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("update info fail: {}", e)
                }
            });
        }
        Err(e) => eprintln!("parse info fail: {}", e.to_string())
    }
}


// tcp handler

pub async fn handle_request_area(json_data: &JsonValue, stream: &mut TcpStream, config: &Config, data: Arc<Mutex<AreaFilesData>>) {
    let mut json_reply = JsonValue::new_object();
    let mut check_ok = true;
    if !json_data.has_key("file_info") {
        check_ok = false;
        json_reply["msg_type"] = JsonValue::from("error");
        json_reply["statu"] = JsonValue::from(-1);
        json_reply["message"] = JsonValue::from("without key 'file_info'");
    } else if !json_data.has_key("tokens") {
        check_ok = false;
        json_reply["msg_type"] = JsonValue::from("error");
        json_reply["statu"] = JsonValue::from(-1);
        json_reply["message"] = JsonValue::from("without key 'tokens'");
    }

    if !check_ok {
        send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await;
        return;
    }
    // ignore the token here...

    let mut download_file_info = None;
    match FileInfo::from_json(&json_data["file_info"]) {
        Ok(info) => download_file_info = Some(info),
        Err(e) => {
            check_ok = false;
            json_reply["msg_type"] = JsonValue::from("error");
            json_reply["statu"] = JsonValue::from(-1);
            json_reply["message"] = JsonValue::from("invalid file_info");
        }
    }

    if !check_ok {
        send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await;
        return;
    }
    // ignore check the file token...

    let mut filepath = String::new();
    match std::path::Path::new(config.get_shared_path())
        .join(&download_file_info.as_ref().unwrap().path)
        .to_str() {
        Some(p) => filepath = p.to_string(),
        None => {
            check_ok = false;
            json_reply["msg_type"] = JsonValue::from("error");
            json_reply["statu"] = JsonValue::from(-1);
            json_reply["message"] = JsonValue::from("invalid path");
        }
    }
    if !check_ok {
        send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await;
        return;
    }

    match fs::OpenOptions::new().read(true).open(filepath) {
        Ok(mut file) => {
            let mut send_file_info = String::new();
            let info_file_path = std::path::Path::new(config.get_shared_path())
                .join(".info_dir")
                .join(format!("{}{}", &download_file_info.as_ref().unwrap().path, "_info.json"))
                .as_path()
                .to_str().unwrap().to_string();

            match read_to_string(
                &info_file_path
                    // .to_str()
                    // .unwrap_or("")
            ) {
                Ok(info) => send_file_info = info,
                Err(e) => {
                    eprintln!("read file fail: {}", e);
                    json_reply["msg_type"] = JsonValue::from("error");
                    json_reply["statu"] = JsonValue::from(0);
                    json_reply["message"] = JsonValue::from("couldn't get the info from the source");
                    send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await;
                    return;
                }
            }
            json_reply["msg_type"] = JsonValue::from("send_area");
            json_reply["statu"] = JsonValue::from(0);
            json_reply["message"] = JsonValue::from("");
            match json::parse(&send_file_info) {
                Ok(info_json) => {
                    json_reply["file_info"] = info_json;
                }
                Err(e) => {
                    json_reply["msg_type"] = JsonValue::from("error");
                    json_reply["statu"] = JsonValue::from(-1);
                    json_reply["message"] = JsonValue::from("source host error");
                    eprintln!("read source info fail.");
                    return;
                }
            }
            match send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("send error: {}", e);
                    return;
                }
            }
            let mut buf = [0u8;4096];
            loop {
                match file.read(&mut buf) {
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }
                        match stream.write_all(&buf[..size]).await {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("send file error: {}", e);
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("send file error: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            json_reply["msg_type"] = JsonValue::from("error");
            json_reply["statu"] = JsonValue::from(-1);
            json_reply["message"] = JsonValue::from("can't get the file");
            eprintln!("couldn't get file: {}", e);

            match send_tcp_packet(stream, &construct_tcp_packet(&json_reply)).await {
                Ok(_) => {}
                Err(e) => eprintln!("couldn't send: {}", e)
            }
        }
    }
}

pub async fn handle_receive_file(stream: &mut TcpStream, config: &Config, data: Arc<Mutex<AreaFilesData>>) {

}

// client handler

pub async fn handle_update_file_info(config: &Config, stream: &mut TcpStream) {
    match send_get_file_information(config).await {
        Ok(_) => {},
        Err(e) => {
            match stream.write_all(format!("couldn't send request: {:?}", e).as_bytes()).await {
                Ok(_) => {},
                Err(e) => eprintln!("response fail: {}", e.to_string())
            }
        }
    }
}

pub async fn handle_list_my(data: Arc<Mutex<AreaFilesData>>, stream: &mut TcpStream) {
    let d = serde_json5::to_string(&data.lock().await.my_files).unwrap();
    let my_info = json::parse(&d).unwrap();
    let mut reply_json = JsonValue::new_object();
    reply_json["statu"] = JsonValue::from(0);
    reply_json["info"] = my_info;
    reply_json["msg"] = JsonValue::from("");
    match send_tcp_packet(stream, &construct_tcp_packet(&reply_json)).await {
        Ok(_) => {}
        Err(e) => eprintln!("response fail: {}", e.to_string())
    }
}

pub async fn handle_list_other(data: Arc<Mutex<AreaFilesData>>, stream: &mut TcpStream) {
    let mut container = vec![];
    for info in data.lock().await.other_files.iter() {
        for i in info.1 {
            container.push(i.clone());
        }
    }
    let d = serde_json5::to_string(&container).unwrap();
    let my_info = json::parse(&d).unwrap();
    let mut reply_json = JsonValue::new_object();
    reply_json["statu"] = JsonValue::from(0);
    reply_json["info"] = my_info;
    reply_json["msg"] = JsonValue::from("");
    match send_tcp_packet(stream, &construct_tcp_packet(&reply_json)).await {
        Ok(_) => {}
        Err(e) => eprintln!("response fail: {}", e)
    }

}

pub async fn handle_download(cmd: &str, data: Arc<Mutex<AreaFilesData>>, stream: &mut TcpStream, config: &Config) {
    let args = cmd.split(" ").collect::<Vec<_>>();
    if args.len() < 4 {
        match stream.write_all("parameters error".as_bytes()).await {
            Ok(_) => {}
            Err(e) => eprintln!("send to client fail: {}", e)
        }
    }
    let (ip, filename) = (args[2], args[3]);
    let mut reply_json = JsonValue::new_object();
    let other_files = &data.lock().await.other_files;
    let mut infos = None;
    for u in other_files {
        match &u.0 {
            &user::User::UserLAN {ip: uip, ..} if ip == uip => {
                infos = Some(u.1)
            }
            _ => {}
        }
    }
    // infos = Some(other_files);

    if infos.is_none() {
        reply_json["statu"] = JsonValue::from(-1);
        reply_json["msg"] = JsonValue::from("unknown ip");
        match send_tcp_packet(stream, &construct_tcp_packet(&reply_json)).await {
            Ok(_) => {
                return;
            }
            Err(e) => {
                eprintln!("send message to client fail: {}", e);
                return;
            }
        }
    }

    let infos = infos.unwrap();
    let mut info = None;
    for ufi in infos {
        if ufi.path == filename {
            info = Some(ufi);
        }
    }
    if info.is_none() {
        reply_json["msg_type"] = JsonValue::from("error");
        reply_json["statu"] = JsonValue::from(-1);
        reply_json["message"] = JsonValue::from("unknown file, please fresh the data");
        match send_tcp_packet(stream, &construct_tcp_packet(&reply_json)).await {
            Ok(_) => {
                return;
            }
            Err(e) => {
                eprintln!("send message to client fail: {}", e);
                return;
            }
        }
    }
    let info = info.unwrap();

    reply_json["msg_type"] = JsonValue::from("request_area");
    reply_json["file_info"] = json::parse(&serde_json5::to_string(info).unwrap()).unwrap();
    reply_json["tokens"] = JsonValue::new_array();

    let mut ctcp = None;
    match TcpStream::connect(format!("{}:{}", ip, config.get_host_port())).await {
        Ok(tcp) => ctcp = Some(tcp),
        Err(e) => {
            eprintln!("connect to {} fail: {}", ip, e);
            return;
        }
    }
    let mut ctcp = ctcp.unwrap();
    match send_tcp_packet(&mut ctcp, &construct_tcp_packet(&reply_json)).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("send request to {} fail: {}", ip, e);
        }
    }

    handle_download_file(&mut ctcp, info, config).await;
}

async fn handle_download_file(stream: &mut TcpStream, info: &FileInfo, config: &Config) {
    let mut json_str_size_buf = Vec::from([0u8; 16]);
    if let Err(e) = stream.read_exact(&mut json_str_size_buf).await {
        eprintln!("read from the stream fail: {}", e);
        return;
    }

    let mut json_str_size = 0usize;
    match String::from_utf8_lossy(&json_str_size_buf).parse::<u32>() {
        Ok(s) => json_str_size = s as usize,
        Err(e) => {
            eprintln!("download file fail: {}", e);
        }
    }

    let mut json_buf = Vec::new();
    json_buf.resize(json_str_size as usize, 0);
    // json_buf.reserve(json_str_size as usize);

    if let Err(e) = stream.read_exact(&mut json_buf).await {
        eprintln!("get json info fail: {}", e);
        return;
    }

    let json_str = String::from_utf8_lossy(&json_buf);
    let mut json_data = JsonValue::Null;
    match json::parse(&json_str) {
        Ok(d) => json_data = d,
        Err(e) => eprintln!("parse json fail: {}", e)
    }

    if !json_data.has_key("msg_type") {
        eprintln!("invalid data from other host");
        return;
    }

    let fsize = json_data["file_info"]["size"].as_u64().unwrap() as usize;
    let mut curr_size = 0usize;
    let mut buf: Vec<u8> = vec![];
    buf.resize(4096, 0);
    let filename = info.path.split("/").last().unwrap().to_string();
    let filepath = std::path::Path::new(config.get_download_path()).join(&filename).as_os_str().to_str().unwrap().to_string();
    let mut output = fs::OpenOptions::new().create_new(true).write(true).truncate(false).open(&filepath);
    match output {
        Ok(ref mut output) => {
            loop {
                match stream.read(&mut buf).await {
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }
                        curr_size += size;
                        if let Err(e) = output.write_all(&buf[..size]) {
                            eprintln!("write file {} fail: {}", &filepath, e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("read data from other host fail: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("create file {} fail: {}", &filepath, e);
            return;
        }
    }
    if curr_size != fsize {
        eprintln!("download file fail");
        return;
    }

    if output.unwrap().flush().is_err() {
        eprintln!("write to file {} fail", &filepath);
        return;
    }
}
