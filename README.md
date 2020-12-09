# Prox

## Purpose

Prox is a set of executables `prox-server` and `prox-client` with the primary purpose of providing an easy to use, non-intrusive method for queueing Proxmox (or other QEMU) VMs to start after the first VM in the queue has shut itself down. The use case for this software package was to aid with switching between Proxmox VMs that involved GPU passthrough. i.e. if I need to switch to my windows desktop VM I want to avoid having to log into the web UI via another computer and starting up the VM after I have shutdown my Linux desktop VM. `prox-client` allows one to add VMs to the `prox-server` running on the host machine. `prox-server` will be checking the status of the first VM in the queue, and when it is shutdown, it will start the next VM in the queue, thus allowing for the seemless transition between GPU-passthrough VMs without the need for an external device.

## Installation

TODO: add in prebuilt executables with a simple installer

## Usage


## Build from source

Clone the repo

```bash
git clone https://github.com/ProjectInitiative/prox.git
```

### Build the server

```bash
cd prox/prox-server
rustup target add x86_64-unknown-linux-musl
rustup override set nightly
cargo build --release
```

This will produce a `prox-server` executable under `prox-server/target/release/` directory
The MUSL target needs to be added since most Proxmox installations do not ship with the bleeding edge version of glibc. The MUSL target will create a truly static binary for the server. 

### Build the client

```bash
cd prox/prox-client
rustup override set nightly
cargo build --release
```
This will produce a `prox-client` executable under `prox-client/release/` directory

### Cross-compiling the client for Windows

To cross compile the client for Windows you will need to add the Windows tool chain to your system. The following commands will get you setup on a Debian based system:

First add the Windows Rust tool chain

```bash
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```

Install the Windows mingw gcc linker

```bash
sudo apt install gcc-mingw-w64-x86-64
```

Lastly build the release version of the software

```bash
 cargo build --target x86_64-pc-windows-gnu --release
```
This will produce a `prox-client.exe` windows executable under `prox-client/target x86_64-pc-windows-gnu/release/` directory


## Additional information

  * [Install rust toolchain](https://rustup.rs/)

  * [Rust Debugging VSCode](https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/)

  * Makeself was used to create universal installer (for systems like Unraid)



Example command:
```bash
curl localhost:8000/api/v1/add-vm-to-queue -H "Content-Type: application/json" --data '{"title":"test1","id":45,"description":"testVM"}'
```

## TODOS:

  * Build client program - possibly GUI
  * Add in API key support
  * Config updates:
    * Change port number
    * Which mode? dynamic queue or alternative static list?
    * API endpoints to check modes, switch modes, and update modes
    * Store queue and static list to disk for powerloss retention
  * Add logging