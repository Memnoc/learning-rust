## Temperature Converter

A simple Rust utility to convert Farenheit to Celsius and the other way around.

**What could we improve**

- Use `Enums` for the temperatures

```rust
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

// chatgpt still insists on passing strings around
impl TemperatureUnit {
    fn from_str(unit: &str) -> Option<Self> {
        match unit.to_uppercase().as_str() {
            "C" => Some(TemperatureUnit::Celsius),
            "F" => Some(TemperatureUnit::Fahrenheit),
            _ => None,
        }
    }
}
```

- Do not return `0.0` but panic instead - done
- Write a `from_str` associated method

```rust
impl TemperatureUnit {
  fn from_str(unit: &str) -> Option<Self> {
    // match on unit, return None for an invalid unit
  }
}
```
