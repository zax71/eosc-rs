use std::{
    net::{IpAddr, UdpSocket},
    vec,
};

use rosc::{OscError, OscMessage, OscPacket, OscType};
use snafu::{ResultExt, Snafu};

pub struct EosDesk {
    pub desk: (IpAddr, u16),
    socket: UdpSocket,
}

#[derive(Debug, Snafu)]
pub enum EosDeskError {
    #[snafu(display("failed to create EOS OSC UDP Socket on {to:#?}"))]
    FailedUDPSocket {
        source: std::io::Error,
        to: (IpAddr, u16),
    },

    #[snafu(display("failed to encode OSC packet with destination {address}"))]
    FailedPacketEncoding { source: OscError, address: String },

    #[snafu(display("failed to send UDP packet with data: {data:#?} to {to:#?}"))]
    FailedUDPSend {
        source: std::io::Error,
        data: Vec<u8>,
        to: (IpAddr, u16),
    },
}

impl EosDesk {
    pub fn new(host: (IpAddr, u16), desk: (IpAddr, u16)) -> Result<Self, EosDeskError> {
        let socket = UdpSocket::bind(host).context(FailedUDPSocketSnafu { to: host })?;

        Ok(Self { desk, socket })
    }

    pub fn execute_cmd(&self, command: &str) -> Result<(), EosDeskError> {
        let address = "/eos/newcmd";
        let args = vec![OscType::String(format!("{command} ENTER"))];

        let packet = OscPacket::Message(OscMessage {
            addr: address.to_string(),
            args: args,
        });

        let buf = rosc::encoder::encode(&packet).context(FailedPacketEncodingSnafu { address })?;
        println!("{:#?}", buf.as_slice());

        self.socket
            .send_to(&buf.as_slice(), self.desk)
            .context(FailedUDPSendSnafu {
                data: buf,
                to: self.desk,
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_send_cmd() {
        let desk: EosDesk = EosDesk::new(
            (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8001),
            (IpAddr::V4(Ipv4Addr::new(192, 168, 122, 95)), 8000),
        )
        .expect("Failed to init desk");
        desk.execute_cmd("GROUP 7 AT 5")
            .expect("Failed to send command");

        assert!(true)
    }
}
