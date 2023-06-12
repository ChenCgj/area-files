use tokio::io;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::runtime::Handle;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use area_files_lib::{Server, user, file_mgr, protocol};
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

async fn update_info(msg: &protocol::MsgReplyArea, data: Arc<Mutex<AreaFilesData>>) -> Result<(), String> {
    let mut data = data.lock().await;
    let user = &msg.user;
    data.update_other_files(user, &msg.info);
    Ok(())
}


// udp handler
pub fn handle_query_area(handle: &Handle, addr: SocketAddr, config: &'static Config) {
    handle.spawn(async move {
        let mut addr = addr;
        match reply_get_file_information(config, &mut addr).await {
            Ok(_) => {}
            Err(e) => eprintln!("send reply error: {:?}", e)
        }
    });
}

pub fn handle_reply_area(handle: &Handle, message: &str, data: Arc<Mutex<AreaFilesData>>) {
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
    match stream.write_all(d.as_bytes()).await {
        Ok(_) => {},
        Err(e) => eprintln!("response fail: {}", e.to_string())
    }
}