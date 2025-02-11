<div align="center">

# PixPurge - pxp

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![Rust](https://github.com/danczw/pixpurge/actions/workflows/rust-ci.yml/badge.svg)

**Purge metadata from pictures and safeguard your personal information. A CLI written in Rust.**

</div>

------------

<br>

_short tool description_

_flag description_

<br>

------------

<br>

## 🛠️ Development

### Build and Run

Clone the repository and run the following command to build the project:

```bash
cargo build
```

To use pixpurge, use the following command:

```bash
cargo run -- <flags> <folder-path>
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

<br>

------------

<br>

## 📖 Usage

Either build the project yourself or download the latest release from the [releases page](https://github.com/danczw/pixpurge/releases).

Build the project yourself:

```bash
cargo build --release
```

or directly install and add the binary to your PATH in order to use it globally:

```bash
cargo install --path .
```

Then, simply start chatting via

```bash
pxp -V
```

<br>

------------

<br>

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
