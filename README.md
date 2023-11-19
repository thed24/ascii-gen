# Rust ASCII Art Converter

This Rust tool converts images into ASCII art. It takes an image file as input, resizes it according to specified width and height parameters, and then converts the image into ASCII characters.

## Installation

Use cargo to quickly and easily install ascii-gen:

```bash
cargo install ascii-gen
```

## Usage

Run the tool:

```bash
ascii-gen --file path/to/your/image.jpg
```

Optional parameters:

- `--width`: Set the width of the output (default: 80).
- `--height`: Set the height of the output (default: 80).

Example:

```bash
ascii-gen --file path/to/your/image.jpg --width 120 --height 60
```

## Dependencies

- [`clap`](https://docs.rs/clap/) - Command-line argument parser for Rust.
- [`image`](https://docs.rs/image/) - Image processing library for Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Special thanks to the Rust community for their support and contributions.
