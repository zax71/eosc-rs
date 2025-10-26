# eosc-rs

[![Doccumentation](https://img.shields.io/docsrs/eosc-rs)](https://docs.rs/eosc-rs/latest/eosc_rs/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![CI](https://github.com/zax71/eosc-rs/actions/workflows/publish.yml/badge.svg?branch=main)](https://github.com/zax71/eosc-rs/actions/workflows/publish.yml)
[![Crates.io License](https://img.shields.io/crates/l/eosc-rs)](https://github.com/zax71/eosc-rs/blob/main/LICENSE)

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
