extern crate chainpoint;

use  serde_json::to_string_pretty;

fn main() {
    let hashes = vec![
        "5b2ed30545431fc087ca0ebe766bc69967b5e492d79ea271e7b0144edfab5204".to_string(),
        "5e70190bb382f264e1b75940f1e1f49d789431c7ef4b67aa027d1e29217469c2".to_string(),
        "90adf4be4c47e84d73149bc13e459dd27c89a639d309f9339e8ef7d9d77eb472".to_string(),
    ];

    println!("Hashes: {}\n", to_string_pretty(&hashes).unwrap());

    let root = chainpoint::merkle_root(hashes);

    println!("Root: {}", root);
}
