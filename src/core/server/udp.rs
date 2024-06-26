use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::core::service::PacketHandler;
use crate::protocol::NetPacket;

pub async fn start(main_udp: Arc<UdpSocket>, handler: PacketHandler) {
    loop {
        let mut buf = vec![0u8; 65536];
        match main_udp.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let handler = handler.clone();
                let udp = main_udp.clone();
                tokio::spawn(async move {
                    match NetPacket::new(&mut buf[..len]) {
                        Ok(net_packet) => {
                            if let Some(rs) = handler.handle(net_packet, addr, &None).await {
                                if let Err(e) = udp.send_to(rs.buffer(), addr).await {
                                    log::error!("{:?} {}", e, addr)
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("{:?} {}", e, addr)
                        }
                    }
                });
            }
            Err(e) => {
                log::error!("{:?}", e)
            }
        }
    }
}
