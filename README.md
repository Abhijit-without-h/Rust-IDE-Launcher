# IDE Launcher

IDE Launcher is a simple Rust program that provides a command-line interface for quickly launching different Integrated Development Environments (IDEs) and code editors.

## Features

- Launch Cursor, Visual Studio Code, or IntelliJ IDEA from a single interface
- Easy-to-use command-line menu
- Written in Rust for performance and reliability

## Prerequisites

Before you can use IDE Launcher, ensure you have the following installed:

- Rust programming language (https://www.rust-lang.org/tools/install)
- The IDEs or editors you want to launch (Cursor, Visual Studio Code, IntelliJ IDEA)

Make sure the IDEs/editors are properly installed and accessible from the command line.

## Installation

1. Clone this repository or download the `ide_launcher.rs` file.
2. Open a terminal and navigate to the directory containing `ide_launcher.rs`.
3. Compile the program using Rust:

   ```
   rustc ide_launcher.rs
   ```

4. This will create an executable file named `ide_launcher` (or `ide_launcher.exe` on Windows).

## Usage

1. Run the program:
   - On Unix-like systems: `./ide_launcher`
   - On Windows: `ide_launcher.exe`

2. You'll see a menu with options to launch different IDEs/editors.
3. Enter the number corresponding to the IDE/editor you want to launch.
4. The program will attempt to launch the selected application.

## Customization

The program uses default command-line commands to launch each IDE/editor:

- Cursor: `cursor`
- Visual Studio Code: `code`
- IntelliJ IDEA: `idea`

If these commands don't work on your system, you may need to modify the `launch_program` function in `ide_launcher.rs` to use the correct commands or full paths to the executables.

## Contributing

Contributions to IDE Launcher are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Contact

If you have any questions or feedback, please open an issue in the GitHub repository.

Enjoy using IDE Launcher!
