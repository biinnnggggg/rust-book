# Chapter 1

- Learned how to install Rust.
- `rustup`: command line tool for managing Rust versions and related tools.
- `cargo`: build system and package manager. Handle dependencies and whatnot.
  - `cargo new`: creates a new project
  - `cargo build`: build a project into `.exe` and store it `target/debug` directory.
  - `cargo run`: build project and run `.exe` in one step
  - `cargo check`: build without producing binary to check for errors.
  - `cargo build --release`: compile with optimizations when project is ready for release. Store `.exe` output in `target/release` directory.