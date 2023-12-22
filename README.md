# Chainpoint rust

## Description

A rust library to create and verify Chainpoint proofs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chainpoint = "0.1.0"
```

## Usage

```rust
use chainpoint::merkle_root;

fn main() {
    let hashes = vec![
        "5b2ed30545431fc087ca0ebe766bc69967b5e492d79ea271e7b0144edfab5204".to_string(),
        "5e70190bb382f264e1b75940f1e1f49d789431c7ef4b67aa027d1e29217469c2".to_string(),
        "90adf4be4c47e84d73149bc13e459dd27c89a639d309f9339e8ef7d9d77eb472".to_string(),
    ];

    let root = merkle_root(hashes);

    println!("Root: {}", root);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
