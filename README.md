# bta30ctl

> Because working 12 hours to develop this tool was worth not spending the 15 seconds opening my phone's app everytime I want to change my music's volume.

(Very barebone) utility to interact with a [Fiio BTA 30](https://www.fiio.com/bta30) and its [Pro](https://www.fiio.com/bta30pro) variant.

Can only control the volume for now (which is probably the most used use case).

## Requirements

- A Bluetooth stack that can work with [btleplug](https://github.com/deviceplug/btleplug)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Install

> Not on crates.io for now, is planned

```
git clone https://github.com/ShinySaana/bta30ctl
cd bta30ctl
cargo install --path .
```

## Usage

```
bta30ctl <volume>
```