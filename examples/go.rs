extern crate eosc_rs;

use std::net::{IpAddr, Ipv4Addr};

use eosc_rs::eos_desk::EosDesk;

fn main() {
    let desk: EosDesk = EosDesk::new(
        (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8001),
        (IpAddr::V4(Ipv4Addr::new(192, 168, 122, 95)), 8000),
    )
    .expect("Failed to init desk");

    desk.go().expect("Failed to go to next cue on the desk")
}
