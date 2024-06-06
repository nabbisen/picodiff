# Pico Diff

Tiny GUI app to compare text easily. Lightweight as feather, handy in palm and has cross-platform support. Written in [Rust](https://www.rust-lang.org/) along with [fltk-rs](https://github.com/fltk-rs/fltk-rs) (Rust's binding for [FLTK](https://www.fltk.org/)).

[![crates.io](https://img.shields.io/crates/v/picodiff?label=latest)](https://crates.io/crates/picodiff)
[![Documentation](https://docs.rs/picodiff/badge.svg?version=latest)](https://docs.rs/picodiff/latest)
[![Dependency Status](https://deps.rs/crate/picodiff/latest/status.svg)](https://deps.rs/crate/picodiff/latest)
[![License](https://img.shields.io/github/license/nabbisen/picodiff)](https://github.com/nabbisen/picodiff/blob/main/LICENSE)

## Usage

Just run the binary for your platform, either of Windows, MacOS, Linux. The window will start so put text at input fields on the form. The comparison result will appear.

![comparison result](.docs-assets/comparison-result.png)

## Acknowledgements

Depends on `Rust` programming language, `FLTK` cross-platform GUI toolkit, `fltk-rs` and [`similar`](https://github.com/mitsuhiko/similar) diffing library.
