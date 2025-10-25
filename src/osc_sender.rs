use std::net::{IpAddr, UdpSocket};

use rosc::{OscError, OscMessage, OscPacket, OscType};
use snafu::{ResultExt, Snafu};

pub struct OscSender {
    target: (IpAddr, u16),
    socket: UdpSocket,
}

#[derive(Debug, Snafu)]
pub enum OscSenderError {
    #[snafu(display("failed to bind UDP socket to {to:#?}"))]
    FailedUDPBind {
        source: std::io::Error,
        to: (IpAddr, u16),
    },

    #[snafu(display("failed to encode OSC packet with destination {address}"))]
    FailedOSCPacketEncoding { source: OscError, address: String },

    #[snafu(display("failed to send UDP packet with data: {data:#?} to {to:#?}"))]
    FailedUDPSend {
        source: std::io::Error,
        data: Vec<u8>,
        to: (IpAddr, u16),
    },
}

impl OscSender {
    /// Creates a new OSCSender
    /// Binds to "bind", should normally be 0.0.0.0 at whatever port you would like
    /// Sends messages to "target", the IP address and port should match that of what you are sending to
    pub fn new(bind: (IpAddr, u16), target: (IpAddr, u16)) -> Result<Self, OscSenderError> {
        let socket = UdpSocket::bind(bind).context(FailedUDPBindSnafu { to: bind })?;

        Ok(Self { target, socket })
    }

    pub fn send_osc(&self, address: String, args: Vec<OscType>) -> Result<(), OscSenderError> {
        let packet = OscPacket::Message(OscMessage {
            addr: address.clone(),
            args: args,
        });

        let data = rosc::encoder::encode(&packet).context(FailedOSCPacketEncodingSnafu {
            address: address.to_string(),
        })?;

        self.socket
            .send_to(data.as_slice(), self.target)
            .context(FailedUDPSendSnafu {
                data,
                to: self.target,
            })?;

        Ok(())
    }
}
