# rustyFetch

This is a Rust program inspired by neofetch that displays system information and ASCII art based on the system configuration. The program uses the `sysinfo` crate to gather system information.

## Installation

To use this program, you need to have Rust installed on your system. You can install Rust by following the instructions on the official Rust website: [Rust Installation](https://www.rust-lang.org/tools/install)

Once Rust is installed, you can clone this repository and build the program using the following commands:

```
git clone <repository_url>
cd <repository_directory>
cargo build --release
```

The built executable will be available in the `target/release/` directory.

## Usage

Run the program from the terminal to display system information and ASCII art:

```
./neofetch-rust
```

The program will gather the system information, including the operating system, hostname, uptime, number of installed packages, kernel version, window manager, CPU, memory, shell, and color palette.

Additionally, the program will read an ASCII art configuration file and display it on the top of the system information table.
