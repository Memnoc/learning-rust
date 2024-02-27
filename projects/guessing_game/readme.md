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

Notice that for this particular program, we could have also written the same code as a one-liner:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Also notice that `read_line()` takes a user input and returns an Enum called [Result](https://doc.rust-lang.org/std/result/enum.Result.html)
[Enums](https://doc.rust-lang.org/book/ch06-00-enums.html) aka Enumerations are types with their possibe variants defined (very similar to objects)

```rust
pub enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

`Result` here is a type that represents either success (`Ok`) or failure (`Err`)
Here is an example implementation

```rust
// this is a success
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_ok(), true);

// this is an error
let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_ok(), false);
```

The `{}` crab pinchers (placeholders)
These are placeholders for values and experessions. When they are for values they containt the variable and when they are expressions they are empty:

```rust
let x = 5;
let y = 10;

println("x is {x} and y + 2 is ", y + 2);
```

**[dependencies] and crates**
These are all declared in `Cargo.toml` and they follow semantic versioning. If you type something like `0.8.15` it actually means `^0.8.15` so the current version and above
After you add dependency, run `cargo build`
All the crates live in `crates.io`

**shadowing**
When we do `let mut guess` and then after `let guess` we are using a feature of Rust called shadowing.
It basically lets you use the same variable name without it being _illegal_ and it's a technique often used for type conversion scenarios.
