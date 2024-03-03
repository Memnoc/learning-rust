## Control Flow

`if` expressions are the most common control flow. We also have `loop` `while` and `for`.

**if Expressions**

It's basically a `true` or `false` basic system with an optional `else` and also `else if`.

- the `if` **must** evaluate to a `bool`
- too many `if`'s can clutter, that's a good indication to use a `match`

```rust
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}
```

- because `if` is an expression you can use on the right side of a `let` statement

```rust
fn main() {
  let condition = true;
  let number = if condition {5} esle {6}; //if evaluates to 5 and else evaluates to 6

  println!("The value of number is: {number}");
}
```

- when used like this, the final value of the expression is the final else block.

**Repetition with Loops**
Three kinds of loops: `loop`, `while` and `for`.

**Loop**
Useful when you need to retry an operation you know might fail, such as whether a thread has completed its job.

- you have `break` and `continue` like in any other language

```rust
fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {result}");
}
```
