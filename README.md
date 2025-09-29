# eosc-rs

A wrapper for EOS OSC commands in Rust.

## Basic Usage

```rs
eos_desk: EosDesk = EosDesk::new("<ip address>", port) // Hangs until a ping is returned, or returns None at timeout

eos_desk.channel(5).at(50)
eos_desk.channel(5).remdim()
eos_desk.channel(5).pan(180)
eos_desk.channel(5).tilt(180)


```