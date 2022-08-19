# Description

This repo is for educational purposes only. It contains homework project on Visitor pattern implementation in Rust.

Visitor is implemented as a type that can be passed to SmartDeviceList::turn_off method and it'll turn off all devices of particular kind.
We can create multiple visitors of different kinds to perform special actions on dedicated device types.

## Test: 

cargo test