#Wolfenstein 3D Style Raycasting Engine

This project is a raycasting engine written in Rust inspired by Fabien Sanglard's 'Game Engine Black Book: Wolfenstien 3D'. I've been following his engine teardown blog for a longtime and managed to buy a copy of his Wolfenstein Book on release. #FirstEd

There isn't much as of yet as this has mostly been a Rust practice project. I started a little bit of work on this over Winter Break to get more familiar with Rust as my senior capstone project involves a lot of Rust.

## TODOS
TODO Automate this installation process for SDL2 https://github.com/Rust-SDL2/rust-sdl2

## Compilation

[Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2) needs to be installed. TODO fetch script for that.

Otherwise `cargo check` and `cargo run` should work fine.

## Running this

``` cargo run --release ```
Invoke this to run the release optimized version.

## Benchmarking

``` cargo run --release ```
vs
``` cargo run ```

I just benchmarked the regular run and --release version and theres a 10x speedup.
