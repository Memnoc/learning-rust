## The Slice Type

It's a reference to a contiguos sequence of elements in a collection rather than the whole collection.

**String Slices**
It's a reference to a part of a `String`.
The pattern is `[starting_index..ending_index]` where `..` is known as `range syntax`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11]; // world here is a slice that contains a pointer to byte at index 6 of s with a length value of 5
```

If you want to start at index 0, you can omit the starting_index

```rust
let s = String::from("hello");

let slice = [0..2]; // is the same as
let slice = [..2]; // shorter
```

If the slice includes the last byte of the string, you can drop the trailing number

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len]; // same as
let slice = &s[3..]; // shorter
```

If you want to take a slice of the entire string, you can drop both numbers

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len]; // same as
let slice = &s[..]; // shorter
```

**String Literal as Slices**

So if we take this example:

```rust
let s = "Hello, world!";
```

The type `s` here is `&str` and it is in fact a slice pointing to **that specific point** of the binary. This is also why **string literals are immutable**
`&str` is an immutable reference.

**String Slices as Parameters**
Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality.

Improvement from `String` to `&str`

```rust
fn first_word(s: &str) -> &str {
```

**Other types of Slices**

We can use more general type of slices too:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[0..3];

assert_eq!(slice, &[2, 3]);
```

This slice type is `&[i32]` and it works in the same way, by storing a reference to the first element and a lenght. This is the most common slice in many collections.
