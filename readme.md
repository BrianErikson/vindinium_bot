# Vindinium Bot

[![Join the chat at https://gitter.im/EmergentOrganization/vindinium_bot](https://badges.gitter.im/EmergentOrganization/vindinium_bot.svg)](https://gitter.im/EmergentOrganization/vindinium_bot?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Bot name: Emergent_Bot

## Getting Started

Install the following:
* [Rust](https://www.rust-lang.org/)
* [Cargo](https://crates.io/install)
* [OpenSSL](https://github.com/sfackler/rust-openssl/blob/master/README.md)

Run the project by navigating to the root directory in a command prompt,
and then type `cargo run`. 

NOTE: You cannot run the program in IntelliJ IDEA's console, as `vindinium::State::pretty_print` evokes a panic on the main thread, causing a halt in the program.

### Windows

Highly suggest running project builds through the MSYS2 environment. 
* Install Rust and Cargo as you normally would, but create your project directory and download git through the MSYS shell to emulate a unix environment. 
* Install OpenSSL via the MSYS shell. The readme for the process is in the OpenSSL link under **Getting Started**.

### Linux Distros

Follow the procedures outlined in the **Getting Started** links.
