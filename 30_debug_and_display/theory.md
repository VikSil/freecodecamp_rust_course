# Debug and Display

* for prinatable types Debug or Display traits need to be implemented
* these traits are automatically implemented for types in the standard library (and only for those types), in `std::fmt::Debug` and `std::fmt::Display` respectively
* Debug trait is implemented via derivable trait
* Display trait must be implemented explicitly

# Printing

* printing is handled by macros implmented in `std::fmt`
* some of ht emacros include:

    * `format!` - writes formatted text to String
    * `print!` - same as format!, but the text is printed to the console (io::stdout)
    * `println!` -  same as print!, with a new line upended
    * `eprint!` - same as format!, but the text is printed to the standard error (io::stderr)
    * `eprintln!` - same as eprint!, with a new line upended

* all of the above parse text in the same fashion
* rustc checks for format correctness at compile time
