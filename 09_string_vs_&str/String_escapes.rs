fn main() {
    // escapes can be used to write bytes by their hexadecimal values
    let byte_escape = "I'm writing Ru\x73\x74!"; // the \x73 is ascii value of "s", and the \x74 is ascii value of "t"
    println!("What are you doing\x3F (\\x3F translates to ?) {}", byte_escape); // using two backslashes means - print a backslash

    // escapes can be used to write Unicode code points
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string =
        "String literals can 
                        span multiple lines.
                        The linebreaks and indentations \
                        can be escaped too.";

    println!("{}", long_string);

    //------------------------------------------------

    let raw_str = r"Escapes don't work here: \x3F \u{211D}"; // this is a raw string, since it is prefixed with r - this means it will ignore all escapes and print everything as a char

    // assert_eq!(raw_str, "Escapes don't work here: ? ℝ"); // because raw strings cannot be escaped, this would cause a panic at runtime (compiler would not catch it)

    // if a raw string contains quotes, then the entire string has to be enclosed in hashtags on either side
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // if a raw string contains quotes and hashtags, then a longer string of hashtags can be used on either side of the string
    let delimiter = r###"A string with "#" in it. And even "##"!"###;
    println!("{}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");
    println!("{}", long_delimiter);
}
