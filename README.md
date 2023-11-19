```
# Rust ASCII Art Converter

This Rust tool converts images into ASCII art. It takes an image file as input, resizes it according to specified width and height parameters, and then converts the image into ASCII characters.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/your_username/rust_ascii_converter.git
cd rust_ascii_converter
```

Build the project using `cargo`:

```bash
cargo build --release
```

## Usage

Run the compiled binary with the path to an image file:

```bash
./target/release/rust_ascii_converter --file path/to/your/image.jpg
```

Optional parameters:

- `--width`: Set the width of the output (default: 80).
- `--height`: Set the height of the output (default: 80).

Example:

```bash
./target/release/rust_ascii_converter --file path/to/your/image.jpg --width 120 --height 60
```

## Dependencies

- [`clap`](https://docs.rs/clap/) - Command-line argument parser for Rust.
- [`image`](https://docs.rs/image/) - Image processing library for Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Special thanks to the Rust community for their support and contributions.
