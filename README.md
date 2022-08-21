# Rust 101
Learning rust from the very beginning

- Following rust docs book
-- https://doc.rust-lang.org/book/

# List of things learned
-> Cargo (buid, run, check): Build tool and package/dependency manager
-> Toml file (Tom's Obvious mini language): Configuration file

-> Rust has an in-built library called Prelude (std::prelude) : https://doc.rust-lang.org/std/prelude/index.html
-> If a type or function isn't on the std::prelude we can import it by typing the 'use' statement

-> Rust is an imperative programming language but it has imports aspects from the fucntional world
-> By default rust variables are immutable, they can't be manipulated. let a = 1 sets the variable 'a' to '1' and you can't manipulate that variable
-> let mut a = 1, gives you the possibility the modify the variable a by declaring it as mutable

-> The '::' represents the association between the crate and its functions
-> String::new() -> represents the function new() from String. It returns an instance of String

-> Rust handles data very safely!
-> Calling io::stdin()::readline().except() handles the 'ok' value returned 
-> io::stdin()::readline().except() handles the 'err' value returned

-> Result is an enumeration 
-> Each state in Result is called a variant

-> To add dependencies (libs, crates, ..) we need to modify the toml file
