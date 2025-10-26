use rosc::OscType;

use crate::osc_sender::OscSender;
use crate::osc_sender::OscSenderError;

pub struct Channel<'a> {
    pub osc_sender: &'a OscSender,
    pub channel: u32,
}

impl Channel<'_> {
    /// Sets this channel's intensity to the specified value
    /// value: The intensity of that channel, in percent.
    pub fn at(&self, value: u8) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            format!("/eos/chan/{}", self.channel),
            vec![OscType::Int(value.into())],
        )
    }

    /// Sets this channel's parameter to the specified value.
    /// param: The parameter to change. e.g: "pan" or "tilt"
    /// value: what to set this parameter to. A value in degrees for pan and tilt
    pub fn param(&self, param: &str, value: i32) -> Result<(), OscSenderError> {
        self.osc_sender.send_osc(
            format!("/eos/chan/{}/param/{param}", self.channel),
            vec![OscType::Int(value)],
        )
    }

    /// Sets a channel's pan value to the specified angle.
    /// degrees: The degree value of the pan of this channel
    pub fn pan(&self, degrees: i32) -> Result<(), OscSenderError> {
        self.param("pan", degrees)
    }

    /// Sets a channel's tilt value to the specified angle
    /// degrees: The degree value of the pan of this channel
    pub fn tilt(&self, degrees: i32) -> Result<(), OscSenderError> {
        self.param("tilt", degrees)
    }

    /// Sets all other channels to zero and leaves this one untouched
    pub fn remdim(&self) -> Result<(), OscSenderError> {
        self.osc_sender
            .send_osc(format!("/eos/chan/{}/remdim", self.channel), vec![])
    }
}
