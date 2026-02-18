# PPM Image Library for Rust 

`ppm_image_rs` is a Rust crate for working with [Netpbm](https://en.wikipedia.org/wiki/Netpbm) PPM files, more 
specifically the `P6` variant at this time.

## Overview 

The `P6` binary format represents R, G, B color components using a byte each (3 * 8 = 24 total bytes) per pixel. The 
binary image data follows a very simple header; the header consists of a magic number, the image dimensions, and the
bit depth.

This crate comprises a `PpmReader` and a `PpmWriter`, which do exactly what they say, and a command line application 
to demonstrate how they are used. 

## Usage 

* Command line application usage: 

```
❯ cargo run --release -- -h
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/ppm_image_rs -h`
Usage: ppm_image_rs <COMMAND>

Commands:
  read            Read a PPM file
  write-gradient  Write a PPM file
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

* Reading a PPM file and showing it in a GUI window 

```
❯ cargo run --release -- read -h
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/ppm_image_rs read -h`
Read a PPM file

Usage: ppm_image_rs read [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>  path to a P6 ppm file [default: ]
  -h, --help                   Print help
❯ cargo run --release -- read -f ./assets/julia_cuda.ppm
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/ppm_image_rs read -f ./assets/julia_cuda.ppm`
image data len = 6220800

```

* Testing the PPM writer by writing a gradient image 

```
❯ cargo run --release -- write-gradient -h
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/ppm_image_rs write-gradient -h`
Write a PPM file

Usage: ppm_image_rs write-gradient [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>  path to a P6 ppm file [default: ]
  -h, --help                   Print help
❯ cargo run --release -- write-gradient -f ./assets/gradient.ppm
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/ppm_image_rs write-gradient -f ./assets/gradient.ppm`
wrote a P6 PPM file, with a gradient, to: "./assets/gradient.ppm"

```


