use std::net::{SocketAddr};
use tokio::io;
use tokio::io::{Interest, ErrorKind};
use tokio::runtime::{Handle, Runtime};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use core::iter::zip;

pub struct Server {
    rt: Runtime,
    listeners: Vec<TcpListener>,
    udp_servers: Vec<UdpSocket>,
    tcp_handler: Vec<fn(TcpStream, SocketAddr, Handle)>,
    udp_handler: Vec<fn(&[u8], SocketAddr, Handle)>
}

impl Server {
    async fn tcp_listen(
        listener: TcpListener,
        func: fn(TcpStream, SocketAddr, Handle),
        handle: Handle
    ) -> io::Result<()> {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    func(stream, addr, handle.clone());
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err(e)
                }
            }
        }
    }

    async fn udp_listen(
        listener: UdpSocket,
        func: fn(&[u8], SocketAddr, Handle),
        handle: Handle
    ) -> io::Result<()> {
        loop {
            if let Err(e) = listener.ready(Interest::READABLE).await {
                return Err(e);
            }
            const UDP_SIZE: usize = 65536;
            let mut buf = [0u8; UDP_SIZE];
            match listener.recv_from(&mut buf).await {
                Ok((buf_size, addr)) => {
                    func(&mut buf[..buf_size], addr, handle.clone());
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err(e);
                }
            }
        }
    }

    pub fn run(self) {
        self.rt.block_on(async {
            let mut handles = vec![];
            let rt_handle = self.rt.handle();
            for (listener, handler) in zip(self.listeners, self.tcp_handler) {
                handles.push(self.rt.spawn(Self::tcp_listen(listener, handler, rt_handle.clone())))
            }
            for (udp_socket, handler) in zip(self.udp_servers, self.udp_handler) {
                handles.push(self.rt.spawn(Self::udp_listen(udp_socket, handler, rt_handle.clone())))
            }
            tokio::join!(async {
                for handle in handles {
                    match handle.await {
                        Ok(_) => {}
                        Err(e) => eprintln!("{:?}", e)
                    }
                }
            });
        });
        println!("Hello area files server");
    }

    pub fn new() -> Result<Server, String> {
        if let Ok(rt) = Runtime::new() {
            Ok(Server {
                rt,
                listeners: vec![],
                udp_servers: vec![],
                tcp_handler: vec![],
                udp_handler: vec![]
            })
        } else {
            Err("create tokio runtime fail".to_string())
        }
    }

    pub fn add_tcp_server(
        &mut self,
        ip: &str,
        port: i16,
        func: fn(TcpStream, SocketAddr, Handle)
    ) -> Result<(), String> {
        self.rt.block_on(async {
            match TcpListener::bind(format!("{}:{}", ip, port)).await {
                Ok(listener) => {
                    self.listeners.push(listener);
                    Ok(())
                },
                Err(e) => Err(e.to_string())
            }
        })?;
        self.tcp_handler.push(func);
        Ok(())
    }

    pub fn add_udp_server(
        &mut self,
        ip: &str,
        port: i16,
        func: fn(&[u8], SocketAddr, Handle)
    ) -> Result<(), String> {
        self.rt.block_on(async {
            match UdpSocket::bind(format!("{}:{}", ip, port)).await {
                Ok(upd_server) => {
                    self.udp_servers.push(upd_server);
                    Ok(())
                }
                Err(e) => Err(e.to_string())
            }
        })?;
        self.udp_handler.push(func);
        Ok(())
    }

}
