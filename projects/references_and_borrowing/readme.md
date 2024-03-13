## References and Borrowing

In other words: how to use a value without taking ownership.
A _reference_ is like a pointer, in that following its address we get to the data stored at that address. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the whole life of that reference.

- reference address leads to the data (like a pointer)
- reference, for its whole existence, points to a valid value (unlike a pointer)

_using and defining a function that has a reference as parameter_

```rust
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1); // refers to the value but does not own it

  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string
  s.len()
  // s is goes out of scope but since it does not have ownership,
  // is not dropped
}
```

So the reference here is represented by the `&` sign, which allow you to refer to some value without taking ownership of it.
When functions have references as parameters instead of the actual values, we won't needto return the values in order to give back ownership, because we never had ownership.

The action of creating a reference is called **borrowing**
Something borrowed, cannot be modified.
**Just as variables are immutable by default, so are references**

```rust
fn main() {
  let s = String::from("Hello");

  change(&s);
}

fn change(some_string: String) {
  some_string.push_str(", world");
}
```

**Mutable references**
There is a way to modify a borrowed value, by using a **mutable reference**

```rust
fn main() {
  let mut s = String::from("Hello"); // make the variable mut

  change(&mut s); // make the variable mutable
}

fn change(some_string: &mut String) { // make the variable mut
  some_string.push_str(", world");
}
```

However, there is one more big limitation: **if you have a mutable reference to a value, you can have no other references to that value**
This allows to prevent `data races` such as when:

- two or more pointers try to access the same data at the same time
- at least one of the pointers is being used to write to the data
- there is no mechanism being used to synchronize access to the data

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time

println!("{}, {}", r1, r2);
```

Rust also **prevents combining mutable and immutable references**

```rust
let mut s = String::from("hello");

let r1 = &s; // ok
let r2 = &s; // ok
let r3 = &mut s; // NOT ok
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

println!("{}, {}, and {}", r1, r2, r3);
```

Notice that a `reference` scope starts from where it is introduced and continues through the last time that reference is used.

```rust
let mut s = String::from("hello");

let r1 = &s; // ok
let r2 = &s; // ok
println!("{} and {}", r1, r2);
// r1 and r2 cannot be used after this point

let r3 = &mut s; // ok
println!("{}", r3);
```

**Dangling references**
It's when you create a pointer to some data that then goes out of scope before the pointer does.
In other words, the compiler ensures the data will not go out of scope before the reference to the data does.

_Example_

```rust
fn main() {
  let reference_to_nothing = dangle();

}

fn dangle() -> &String { // dangle returns a reference to a string
  let s = String::from("hello"); // new string is created

  &s // we return a reference to the String
  // s goes out of scope and it is dropped - DANGER
// error[E0106]: missing lifetime specifier
}
```

The solution is returning `String` directly:

```rust
fn no_dangle() -> {
  let s = String::from("hello");

  s // this return transfers ownership so no problem
}
```
