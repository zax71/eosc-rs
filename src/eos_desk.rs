use std::{net::IpAddr, vec};

use rosc::{OscError, OscType};
use snafu::Snafu;

use crate::{
    channel::Channel,
    osc_sender::{OscSender, OscSenderError},
};

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
    /// Creates an EosDesk that sends messages to the desk at desk
    /// host: What to bind the UDP port to, (0.0.0.0, 8001) is a sensible default
    /// desk: The EOS desk you are connecting to, so the IP of said desk and it's OSC port (8000 by default)
    pub fn new(host: (IpAddr, u16), desk: (IpAddr, u16)) -> Result<Self, OscSenderError> {
        let osc_sender = OscSender::new(host, desk)?;

        Ok(Self { osc_sender })
    }

    /// Executes the stated command. Auto executes by appending ENTER to the end of said command
    /// command: The command to be sent to the desk. e.g: "GROUP 5 FOCUS PALETTE 2"
    pub fn command(&self, command: &str) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            "/eos/newcmd".to_string(),
            vec![OscType::String(format!("{command} ENTER"))],
        )?;

        Ok(())
    }

    /// Creates a channel to be used later.
    pub fn channel(&'_ self, channel: u32) -> Channel<'_> {
        Channel {
            osc_sender: &self.osc_sender,
            channel,
        }
    }

    /// Records the cue at cue number
    /// cue_number: what number is the cue to be recorded? A string to stop floating point errors.
    pub fn record_cue(&self, cue_number: &str, time: f32) -> Result<(), OscSenderError> {
        let command: String = format!("Record Cue {} Time {}", cue_number, time);
        self.command(&command)
    }

    /// Goes to the next cue
    pub fn go(&self) -> Result<(), OscSenderError> {
        self.osc_sender
            .send_osc(" /eos/key/go_0".to_string(), vec![])
    }

    /// Goes to the specified cue
    /// cue: The cue to go to, a string to avoid floating point errors
    pub fn fire(&self, cue: &str) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            " /eos/cue/fire".to_string(),
            vec![OscType::String(cue.to_string())],
        )
    }
}
