```markdown
# date-formatter

[![Crates.io](https://img.shields.io/crates/v/date-formatter.svg)](https://crates.io/crates/date-formatter)
[![Docs.rs](https://docs.rs/date-formatter/badge.svg)](https://docs.rs/date-formatter)

A simple and lightweight date formatter crate for Rust. This crate provides functions for formatting, parsing, and performing calculations on dates.

## Features

* **No dependencies:** This crate is implemented using only the Rust standard library, so it has no external dependencies.
* **Easy to use:** The API is simple and straightforward, making it easy to get started.
* **Lightweight:** The crate is small and has minimal overhead.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
date-formatter = "0.1.0"  # Replace with the latest version
```

Then, you can use the crate in your code:

```rust
use date_formatter::{format_date, parse_date, days_between};
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let formatted_date = format_date(now, "%Y-%m-%d");
    println!("Today is: {}", formatted_date);

    let parsed_date = parse_date("2024-12-29").unwrap();
    println!("Parsed date: {:?}", parsed_date);

    let yesterday = now - std::time::Duration::from_secs(86400);
    let days = days_between(now, yesterday);
    println!("Days between now and yesterday: {}", days);
}
```

## Supported Formatting Symbols

* `%Y`: Year, e.g., "2024"
* `%m`: Month, e.g., "12"
* `%d`: Day, e.g., "29"

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This crate is licensed under the MIT License.
```
