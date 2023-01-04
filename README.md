# rust-playground

#### Personal projects on Rust for learning. Contains quick notes on using Rust and Cargo. Reference to rust book: https://rust-book.cs.brown.edu/title-page.html

### Quick Notes

`Cargo.lock` file - when `cargo build` runs, it will see that lock file exists and use the versions recorded in the lock file. This is to have reproducible builds. If you want to update the dependencies, you can run `cargo update` to update the lock file. This lock file should be checked into Git.

`cargo update` will ignore the lock file and gets the latest versions of dependencies specified in the `Cargo.toml` file and then write those latest downloaded versions into the lock file.

`cargo doc --open` will build documentation provided by all the dependencies locally.

`enum` is something that can have a few definite values. It is like a `struct` but with a few definite values. For example, `std::cmp::Ordering` type is an `enum` with three values: `Less`, `Equal`, and `Greater`. It returns a variant of `Ordering` when you compare two values.

`match` expression is made up of `arms`, which is made up of a pattern to match against. Each arm is looked into in turn, take sequence.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

`use std::io` this will bring the type into scope. This is like `import` in Python.


Rust has a strong static type system but it also has type inference.

Rust can let us shadow the previous value of a variable with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. For example, if we want to convert a string to a number, we can do the following:

```rust
let mut guess = String::new();
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Such that we do not need to create the second variable called `guess_str`.

`Result` enum contains `Err` and `Ok` variants. `Err` variant contains the error message and `Ok` variant contains the result.
