# b64rs

> A command line Base 64 encoder and decoder.

![MIT license](https://img.shields.io/github/license/gomihiko/b64rs)


b64rs is a simple command line tool written in Rust that can encode and decode single lines using base 64 from a command line.

![demonstration of the program in a command prompt](https://i.ibb.co/tCS6YR0/repo-scr.png)

## Building
Clone the repository, then build using Cargo.

    git clone https://github.com/gomihiko/b64rs.git
    cd b64rs
    cargo build
## Usage
macOS/Linux

    b64rs -e 'text to encode'
    b64rs -d 'text to decode'
Windows

    b64rs.exe -e 'text to encode'
    b64rs.exe -d 'text to decode'

Use the command line  `-h` flag to display the options above.
