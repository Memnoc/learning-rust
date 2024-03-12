## References and Borrowing

In other words: how to use a value without taking ownership.
A _reference_ is like a pointer, in that following its address we get to the data stored at that address. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the whole life of that reference.

- reference address leads to the data (like a pointer)
- reference, for its whole existence, points to a valid value (unlike a pointer)

_using and defining a function that has a reference as parameter_

```rust
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
```

So the reference here is represented by the `&` sign, which allow you to refer to some value without taking ownership of it.
