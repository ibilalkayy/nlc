# NLC

NLC (NativeLink Client) is a simple command-line application that allows you to upload files using a subcommand-based interface. The project is built with Rust and uses the `clap` crate for argument parsing.

## Features

- Upload files using the `upload` subcommand.
- Supports specifying files with a `--file` or `-f` flag.

## Usage

### Installation

To install the `nlc` command globally, you can clone this repository and build it yourself using Cargo:

```bash
git clone https://github.com/ibilalkayy/nlc
cd nlc
cargo build --release
```

Then, move the binary to a directory in your `$PATH`:

```bash
sudo mv target/release/nlc /usr/local/bin/
```

Now, you can use the `nlc` command globally.

### Running the Application

After installing, you can use the `upload` subcommand to upload a file. The `--file` or `-f` flag allows you to specify the file path.

#### Example:

```bash
nlc upload --file ./path/to/your/file.txt
```

Or with the short flag:

```bash
nlc upload -f ./path/to/your/file.txt
```

### Output

The program will print the file being uploaded:

```bash
Uploading file: ./path/to/your/file.txt
```

## Contributing

Contributions are welcome! If you'd like to contribute to NLC, please follow these steps:

- Fork the repository from GitHub.
- Create a new branch for your feature or bug fix.
- Make your changes and ensure the code builds properly.
- Submit a pull request with a clear description of the changes.

Please ensure that your contributions adhere to the [Apache License 2.0](LICENSE) and that any code submissions are accompanied by adequate tests and documentation.

## Code of Conduct

You can check out the [code of conduct](CODE_OF_CONDUCT.md) by becoming a good member of this project.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.