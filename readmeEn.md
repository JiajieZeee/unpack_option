# unpack_option
中文请看 **[readme](/readme.md)**

A lightweight Rust library that provides extension methods for the Option type, making it easy to convert Option<T> to Result<T, OptionError>.
If you are using `anyhow`, it is more recommended to use `option.with_context(|| "option can not be null")?`

## Features
- Implements the `OptionExt` trait for Option<T>
- Provides two unpacking methods: `unpack()` and `unpack_for()`
- Custom error type `OptionError` supporting generic errors and parameter-specific errors

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
unpack_option = "0.1.0"

```

## Usage Examples
### Basic Usage
```rust
use unpack_option::OptionExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Successfully unpack Some value
    let value = some_value.unpack()?;
    println!("Unpacked value: {}", value);

    // Unpacking None returns an error
    match none_value.unpack() {
        Ok(_) => println!("This line won't execute"),
        Err(e) => println!("Unpack failed: {}", e), // Output: "Unpack failed: param can not be null"
    }

    Ok(())
}
```

### Specifying Parameter Names
```rust
use unpack_option::OptionExt;

fn process_user(id: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    // Specify parameter name for error message
    let user_id = id.unpack_for("user_id")?;
    println!("Processing user ID: {}", user_id);
    Ok(())
}

fn main() {
    if let Err(e) = process_user(None) {
        println!("Error: {}", e); // Output: "Error: user_id can not be null"
    }
}
```

## Error Types
The `OptionError` enum includes two variants:
- `CanNotBeNull`: Generic error indicating an Option value was None
- `CanNotBeNullFor(&'static str)`: Error containing a parameter name for more specific error messages

## License
This project is licensed under the [MIT License](/license).