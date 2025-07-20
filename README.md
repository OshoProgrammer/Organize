Organize

Organize is a command-line tool written in Rust that automatically sorts files in a folder into subfolders based on their file extensions.

For example, a file named video.mp4 will be moved into a folder named "MP4 files", and report.pdf will go into "PDF files".
Features

    Automatically groups files by their extensions

    Uses the current directory if no path is given

    Simple, fast, and efficient

    Cross-platform compatible (Linux, Windows, macOS)

Usage

Run from the terminal:

organize /path/to/your/folder

Or simply:

organize

This will organize files in the current directory.
Installation

Clone the repository and build using cargo:

git clone https://github.com/OshoProgrammer/organize.git
cd organize
cargo build --release
sudo cp target/release/organize /usr/local/bin/

You can now run organize from any directory in your terminal.
Example

Before running:

Downloads
movie.mp4
song.mp3
report.pdf

After running:

Downloads
MP4 files/movie.mp4
MP3 files/song.mp3
PDF files/report.pdf
How It Works

    Reads the target directory (or uses the current one)

    Loops through each file

    Detects the file's extension

    Creates a folder like "PDF files" or "MP4 files"

    Moves the file into the corresponding folder

License

MIT License
Copyright 2025 Osho Sharma
Future Ideas

    Add a dry-run mode

    Add undo support

    Add options to skip or overwrite existing files

    Customize folder naming format

    Option to sort only specific types (e.g., videos or images)

Contributing

Contributions are welcome.

    Fork the repository

    Make your changes

    Push to your fork

    Open a pull request
# Organize
