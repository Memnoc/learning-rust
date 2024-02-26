## A simple guessing game

**Summary**
This is a simple guessing game in Rust to explore some basic concepts

**Concepts**

This is the source code for a `String` - it's a [Vector](https://doc.rust-lang.org/rust-by-example/std/vec.html)

```rust
pub struct String {
    vec: Vec<u8>,
}
```

`Vectors` are re-sizable arrays. Like slices, their size is not known at compile time, but they can grow or shrink at any time. A vector is represented using 3 parameters:

- pointer to the data
- length
- capacity

The capacity indicates how much memory is reserved for the vector. The vector can grow as long as the length is smaller than the capacity. When this threshold needs to be surpassed, the vector is reallocated with a larger capacity

`std::io` is a [module](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/io/index.html#:~:text=The%20std%3A%3Aio%20module,and%20writing%20input%20and%20output.) and it is also defined as _prelude_

The `std::io` module contains a number of common things you'll need when doing input and output. The most core part of this module is the `Read` and `Write` traits, which provide the most general interface for reading and writing input and output.

`.expect("Failed to read line");` - this method can be used to provide a user-defined message in case of panic
`.unwrap();` - similar to `expect(arg)` excepts it takes no arguments and provides system-defined messages in case of panic

[source](https://dev.to/ssivakumar/rust-expect-vs-unwrap-vs--o7k)

```rust
    // Declaring variable with Ok() variant of Result type
    let a: Result<i32, &str> = Ok(100);

    // expect() method panics with a provided custom message
    println!("Value of variable a => {}", a.expect("Variable cannot be empty"));
```

`unwrap()` - this method is very similar but it provides system-defined messages in case of panic (no arguments)

```rust
    // Declaring variable with Ok() variant of Result type
    let a: Result<i32, &str> = Ok(100);

    // unwrap() method panics with a generic message
    println!("Value of variable a => {}", a.unwrap());
```

**source code for expect()**

```rust
pub fn expect(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed(msg, &e),
        }
    }

```

**The weird ::**

`::` and `String::new()` - this is really weird, and it means the function, in this case `new()` is associated witha a type, in this case `String`. This basically means the new function creates an empty string

```rust
let mut guess = String::new() creates a mutable variable and assigns it to an empty string
```
