use common::payload::{CheckIn, CheckOut, WriterPayload};
use log::error;
use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

pub struct WriterClient {
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
}

impl WriterClient {
    pub fn new(socket: Arc<UdpSocket>, addr: SocketAddr) -> Self {
        Self { socket, addr }
    }

    pub fn send_checkin(&self, checkin: CheckIn) {
        self.send(WriterPayload::CheckIn(checkin));
    }

    pub fn send_checkout(&self, checkout: CheckOut) {
        self.send(WriterPayload::CheckOut(checkout));
    }

    fn send(&self, payload: WriterPayload) {
        let request_bytes = match serde_json::to_vec(&payload) {
            Ok(bytes) => bytes,
            Err(err) => {
                error!("Unable to serialize payload: {}", err);
                return;
            }
        };

        match self.socket.send_to(&request_bytes, self.addr) {
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::WouldBlock => {}
            Err(err) => error!("Unable to send load balancer bytes to writer: {}", err),
        }
    }
}
