
# PNGme
[PNGme](https://jrdngr.github.io/pngme_book/introduction.html) is a command line tool that can encode/decode hidden messages inside png files and display png file data using chunks.
## Installation and Run options
To install and run the application you'll need  [rustup](https://www.rust-lang.org/tools/install)<br />From your command line:
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
### encode

Chunks are identified through their chunk type code which is a 4 letter string (e.g: `tEsT`,`Rust`,`dROp`) , you can create a chunk container using the ``encode`` command:
```sh
# Hide the message "can't see me" inside john_cena.png
$ pngme encode john_cena.png cena "can't see me"
```
### print
If we print the above using the ``print`` command:
```sh
# Print the chunk structure in john_cena.png
$ pngme print john_cena.png
```
We get:
```
{
 [Data Length]: 13
 [Chunk Type]: IHDR
 [Data]:  X
 [CRC32-ISO-HDLC]: 2591457904
}

{
 [Data Length]: 224509
 [Chunk Type]: IDAT
 [Data]: INVALID_UTF8_STRING
 [CRC32-ISO-HDLC]: 754340432
}

{ 
 [Data Length]: 12
 [Chunk Type]: cena
 [Data]: can't see me   <-- our hidden message
 [CRC32-ISO-HDLC]: 477994203
}

{
 [Data Length]: 0
 [Chunk Type]: IEND
 [Data]:
 [CRC32-ISO-HDLC]: 2923585666
}
```
> [!WARNING] 
> PNG files contain the `IHDR`, `IDAT` and `IEND` chunks by default, they do not contain any sort of valid string data, although it's not handled you should NOT tamper with those chunks or use their type codes.

### decode
``print`` is more convenient for viewing chunks but it gets tedious to keep track of one chunk so to view the data inside a chunk you can use the ``decode``command<br />Following up with the above example:
```sh
$ pngme print john_cena.png cena
# Prints the following success message:
# Message hidden within chunk "cena" -> can't see me
```
### remove
Serves the exact opposite role of ``encode`` i.e deletes a chunk identified through it's chunk type code
Following up with the above example:
```sh
$ pngme remove john_cena.png cena
# Prints the following success message:
# Removed chunk container (code: cena)
```
> [!NOTE] 
> `remove` deletes the first chunk with the chunk type code specified and does not delete ALL chunks with said type code.
## For Developers
PNGme stores UTF-8 valid strings inside the `.png` binary by making use of an arbitrarily-sized section in [chunks](src/chunk.rs) known as [chunk data](src/chunk.rs#L32) and references that section using a [chunk type](src/chunk_type.rs#L4), these two chunk components along with length (a 4 byte unsigned integer) allows us to find our messages and extract the strings using their exact length.

PNG files are read and parsed into a [png](src/png.rs#L5) class (a list of [chunks](src/chunk.rs#L29)) in order to properly modify it with the class-specified methods, you can see the algorithms in the [commands module](src/commands.rs).

PNGme has fully-fledged classes for the ``.png`` extension albeit it doesn't make full use of them, it can prove worth for any developer who needs to view/modify pngs.
## Credits
PNGme was made with the following open source packages:
[The PNGme project](https://jrdngr.github.io/pngme_book/introduction.html)
[PNG (Portable Network Graphics) specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
[clap crate (ver4.5.45)](https://crates.io/crates/clap)
