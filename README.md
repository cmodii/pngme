# PNGme
[PNGme](https://jrdngr.github.io/pngme_book/introduction.html) is a command line tool that can encode/decode hidden messages inside png files and display png file data using chunks.
## Installation and Run options
To install and run the application you'll need [rustup](https://www.rust-lang.org/tools/install)<br />From your command line:
```sh
# Install the app with cargo
$ cargo install --git https://github.com/cmodii/pngme-rs

# Run the app from anywhere
$ pngme
```

or clone the repository using [Git](https://git-scm.com/) and build the application
```sh
# Clone this repository
$ git clone https://github.com/cmodii/pngme-rs

# Go into the repository
$ cd pngme-rs

# Build the app
$ cargo build --release

# Go into the build directory
$ cd target/debug

# Run the app
$ pngme
```
## How To Use & Examples
You can run the ``pngme`` app with the ``---help`` flag to get a comprehensive list of the commands:
```sh
$ pngme --help
```
