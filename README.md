# `check-for-unwrap`

This is a Rust program that searches through all the `.rs` files in the current directory and its subdirectories for any usage of `.unwrap()`. If it finds any, it prints the file path, line number, and the line of code with `.unwrap()` highlighted in red. If any `.unwrap()` usage is found, the program exits with a status code of 1.

## Dependencies

This program uses the following crates:

- `std::process`
- `std::fs`
- `std::io`
- `walkdir`
- `regex`
- `crossterm`

## Usage

To use this program, simply run it in the directory you want to check for `.unwrap()` usage:

```bash
cargo run
```

If any `.unwrap()` usage is found, you will see output similar to the following:

```bash
src/main.rs:10:let re = Regex::new(r"\.unwrap\(\)").unwrap()
```

In this example, `.unwrap()` was found on line 10 of `src/main.rs`.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

```
Please replace the license and other details as per your project's requirements. This is just a basic template to get you started. Let me know if you need help with anything else! 😊
