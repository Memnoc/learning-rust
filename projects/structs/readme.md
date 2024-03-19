## Struct

A custom data type that lets you package together and name multiple related values that make up a familiar group. Structs are **comparable in structure and intentions to objects**

**Structs:**

- can be different types
- can name each piece of data (fields)

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

To use the struct, you create an instance of it giving the same values you have used, regardless of the order.

- you can access the fields with dot notation
- you can change the fields adding `mut` to the **entire** instance
- you can return the struct with a function, like any other experession

```rust
// returning the struct as end value of a function
fn build_user(email:String, username:String) -> {
  User {
    active: true,
    username: username,
    email: email,
    sign_in_count: 1,
  }
}
fn main() {
  // instantiation
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

  // accessing
    println!("Username: {}", user1.username);
    println!("Is active: {}", user1.active);
    println!("Email: {}", user1.email);
    println!("Is signed in: {}", user1.sign_in_count);

  // making the instance mutable
    let mut user1 = User {
        active: false,
        username: String::from("someusername345"),
        email: String::from("someoneagain@example.com"),
        sign_in_count: 3,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("Mutable user one email: {}", user1.email);
```
