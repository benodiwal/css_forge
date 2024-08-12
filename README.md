# css_forge
A robust and efficient CSS parsing library for Rust

## 🚀 Features
- `Selector Parsing`: Parse complex CSS selectors, including class, ID, and attribute selectors.
- `Property Parsing`: Easily parse CSS properties and their values.
- `Value Parsing`: Parse various CSS value types, including colors, lengths, etc.
- `Error Handling`: Gracefully manage parsing errors such as invalid selectors and unexpected tokens.

## 📦 Installation
To use `css_forge`, add the following to your Cargo.toml:

```toml
[dependencies]
css_forge = "0.1.0"
```

## Usage

### 1. Basic Parsing Example:
Here's how you can parse a simple CSS snippet using css_forge:
```rust
use css_forge::parser::Parser;
use css_forge::css::Stylesheet;

fn main() {
    let input = r#"
        .container {
            width: 100%;
            max-width: 1200px;
        }
        p { color: #333; }
    "#.to_string();
    
    let mut parser = Parser::new(input);
    
    match parser.parse() {
        Ok(stylesheet) => println!("Parsed Stylesheet: {:?}", stylesheet),
        Err(err) => eprintln!("Parsing error: {:?}", err),
    }
}
```

### 2. Handling Errors:
css_forge gracefully handles common errors during parsing:
```rust
use css_forge::{parser::Parser, errors::CssParseError};

fn main() {
    let input = ".invalid-selector { color: }".to_string();
    let mut parser = Parser::new(input);
    
    match parser.parse() {
        Err(CssParseError::InvalidSelector) => eprintln!("Error: Invalid CSS selector"),
        Err(CssParseError::InvalidValue) => eprintln!("Error: Invalid property value"),
        Ok(stylesheet) => println!("Parsed Stylesheet: {:?}", stylesheet),
        Err(err) => eprintln!("Other parsing error: {:?}", err),
    }
}
```
## 👥 Contributing
Contributions are welcome! Feel free to open issues, submit pull requests, or fork the repository to make improvements.

## 📝 License
This library is open-source and available under the [MIT LICENSE](LICENSE).

`Happy styling with css_forge! 🎨🚀`
