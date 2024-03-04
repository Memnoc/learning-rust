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

**Loops labels**
For inner loops you can have `break` and `continue` which can apply to the innermost loop
You can optionally specify a `loop label` to specify to which loop `break` and `continue` apply.
In the example, the first `break` only pertains the outer loop while the one with the label `'counting_up` only pertains the inner loop.

```rust
fn main() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
    println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count = {count}");
}
```

**While loops**
This is pretty standard as any other language

```rust
fn main() {
  let mut number = 3;

  while number != 0 {
    println!("{number}");

    number -= 1;
  }

  println!("LIFTOFF!!");
}
```

**Looping through collection with for**
This is also pretty standard like any other language. The only different thing is that you can provide a `Range` which make this `for loop` a little more powerful for when you need things like countdowns or running the loop a certain amount of times.

```rust
fn main() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {element}");
  }
}


// with a range

fn main() {
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF");
}
```
