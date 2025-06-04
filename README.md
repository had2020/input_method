## `input_method`

![MSRV](https://img.shields.io/badge/Rust%20MSRV-1.78.0-brightgreen)
[![crates.io](https://img.shields.io/crates/v/input_method.svg)](https://crates.io/crates/input_method)
[![Downloads](https://img.shields.io/crates/d/input_method.svg)](https://crates.io/crates/input_method)

https://crates.io/crates/input_method


A **tiny, opinionated Rust crate** that brings a Python-style input method to your CLI apps. Ideal for quick scripts, REPLs, and simple text-based interactions.

## 📦 Purpose

This crate exists to provide **one simple function**: read user input from `stdin`, with an optional prompt. That’s it. Just like Python’s `input()`.

Perfect for:
- Quick scripts and prototypes ⚡
- Fast little CLI tools 🛠️
- School exercises and coding challenges 🧠

## 🚀 Usage

Add it to your `Cargo.toml`:

```bash
cargo add input_method
```

Basic usage 

```rust
use input_method::input;

fn main() {
    println!("What is your name!");
    let name = input();
    println!("Hello, {}", name);
}
```

# Features
- Simple and ergonomic
- No dependencies (just std :C)
- Works out-of-the-box on Unix based platforms
- Minimal footprint (zero config)

# See Github for Up To Date Docs
Leave a star to be a star! ⭐
- https://github.com/had2020/input_method

PRs welcome, keep it simple.
