# eosc-rs

A wrapper for EOS OSC commands in Rust.

## Basic Usage

```rs
eos_desk: EosDesk = EosDesk::new("<ip address>", port) // Hangs until a ping is returned, or returns None at timeout

eos_desk.channel(5).at(50)
eos_desk.channel(5).remdim()
eos_desk.channel(5).pan(180)
eos_desk.channel(5).tilt(180)

eos_desk.record_cue("5.7", 6.8)
eos_desk.go()

eos_desk.live()
eos_desk.blind()
eos_desk.command("Chan 5 Thru 8 At 100")


```
