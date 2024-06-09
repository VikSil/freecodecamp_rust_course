# Cargo 
Cargo is the official package manager and building tool
it automates tasks, such as:
* creating new projects,
* building binaries,
* testing code,
* managing dependencies

# Crate
A crate is Rusts name for a package/sub-package

crates.io is an online registry of Rust packages

There are two types of crates:
 * binaries (executables)
    * binary root will be src/main.rs
    * there can be unimited number of binary crates in a package, all in `/src/bin` directory 
 * libraries
    * library root will be src/lib.rs
    * there can only be one library crate in a package


A new crate is created via command: `cargo new package-name`.
This will create a package-name directory with .toml file and a src sub-directory with main.rs in it

A new library is created via the same command by adding lib flag: `cargo new library-name --lib`


# Module

Modules are a way of organising code by grouping related items together within a crate. 

Modules are imported using namespaces, to avoid naming collisions.

Modules control privacy/visibility of items, such as functions, structs, enums etc.

Modules can be either directly described in the root file (main.rs or lib.rs), in separate file at the root file level or in as a directory at root file level with several .rs files in it.

[Source Video](https://youtu.be/BpPEoZW5IiY?si=qoRxscB9abyY9CsO&t=37703)