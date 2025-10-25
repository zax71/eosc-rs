use std::{
    net::{IpAddr, UdpSocket},
    vec,
};

use rosc::{OscError, OscMessage, OscPacket, OscType};
use snafu::{ResultExt, Snafu};

use crate::osc_sender::{OscSender, OscSenderError};

pub struct EosDesk {
    osc_sender: OscSender,
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
    pub fn new(host: (IpAddr, u16), desk: (IpAddr, u16)) -> Result<Self, OscSenderError> {
        let osc_sender = OscSender::new(host, desk)?;

        Ok(Self { osc_sender })
    }

    /// Executes the stated command. Auto executes by appending ENTER to the end of said command
    pub fn execute_cmd(&self, command: &str) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            "/eos/newcmd".to_string(),
            vec![OscType::String(format!("{command} ENTER"))],
        )?;

        Ok(())
    }

    pub fn chan_intensity(&self, id: u16, value: u8) -> Result<(), OscSenderError> {
        self.osc_sender
            .send_osc(format!("/eos/chan/{id}"), vec![OscType::Int(value.into())])?;

        Ok(())
    }

    pub fn chan_param(&self, id: u16, param: &str, value: i32) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            format!("/eos/chan/{id}/param/{param}"),
            vec![OscType::Int(value)],
        )?;

        Ok(())
    }
}
