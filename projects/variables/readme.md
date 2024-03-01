## Variables and Mutability

Variables are immutable by default - this means once a value is bound to a name, that value canot be changed anymore.

The compiler flags this example as error:

```rust
fn main() {
  let x = 5;
  println("The value of x is {x}");
  x = 6;
  println("The value of x is {x}");
}
```

The fix for this is to make the variable mutable;

```rust
let mut x = 5;
```

In this way, we can re-assign the value to `x` without any problems

**Constants**
Similar to variable but different in the crucial aspect of managing memory.

- You cannot use `mut` with constants.
- Can declare constants in any scope.
- Can only be assigend to a constant expression, not to a value that might change or that can only be computed at runtime.
- You have to annotate the type.
- They are valid for the entire time a program runs, within the scope they were declared.

```rust
const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;
```

**Shadowing**
You can declare two variables with the same name. The second variable is what the compiler see. This means that the second variable overshadows the first one.

Shadowing is different from `mut` in that:

- it effectively creates a new variable
- we can therefore change the type of the value, but using the same name
- we can perform transformations on the variable but still have it immutable

In the following scenario, shadowing is in action in two different scopes.

```rust
let y = 5;

    let y = y + 5;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}"); // 20
    }

    println!("The value of y in the outer scope is {y}"); // 10
```

This is very useful when you need to do stuff like this:

```rust
let spaces = "  ";
let spaces = spaces.len();
```

But you cannot do this:

```rust
let mut spaces = "  ";
spaces = spaces.len();
```

Because we are using `mut` we cannot change the type of the variable.
