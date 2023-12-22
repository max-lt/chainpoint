use hex::FromHex;
use bitcoin_hashes::sha256::Hash;
use bitcoin_hashes::Hash as _;

fn from_hex(data: &str) -> Hash {
    Hash::from_slice(&Vec::from_hex(data).unwrap()).unwrap()
}

fn hash_256(left: Hash, right: Hash) -> Hash {
    let data = &[left, right].concat();

    Hash::hash(data)
}

pub fn merkle_root(hashes: Vec<String>) -> Hash {
    let mut hashes: Vec<Hash> = hashes.iter().map(|hash| from_hex(hash)).collect();

    while hashes.len() > 1 {
        let mut level = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            let left = hashes[i];
            let right = if i + 1 < hashes.len() {
                hashes[i + 1]
            } else {
                left
            };

            level.push(hash_256(left, right));
        }

        hashes = level;
    }

    hashes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static ANCHORS: [&str; 3] = [
        "5b2ed30545431fc087ca0ebe766bc69967b5e492d79ea271e7b0144edfab5204",
        "5e70190bb382f264e1b75940f1e1f49d789431c7ef4b67aa027d1e29217469c2",
        "90adf4be4c47e84d73149bc13e459dd27c89a639d309f9339e8ef7d9d77eb472",
    ];

    static ROOT: &str = "08b2e94e4d80eeafca41820aacf6d9c5da1a4464667684f09cc344b0f64a507d";

    #[test]
    fn test_from_hex() {
        assert_eq!(
            from_hex(ANCHORS[0]),
            Hash::from_byte_array([
                91, 46, 211, 5, 69, 67, 31, 192, 135, 202, 14, 190, 118, 107, 198, 153, 103, 181,
                228, 146, 215, 158, 162, 113, 231, 176, 20, 78, 223, 171, 82, 4
            ])
        )
    }

    #[test]
    fn test_hash_256() {
        let left = from_hex(ANCHORS[0]);
        let right = from_hex(ANCHORS[1]);

        let hash = hash_256(left, right);

        assert_eq!(
            hash,
            from_hex("73ece99a1952c529b037c83cf3cc229f12cfff37a3972833a9b9c59ccdfc1b9c")
        );
    }

    #[test]
    fn test_merkle_root() {
        let hashes = ANCHORS.iter().map(|a| (a.to_string())).collect();

        let root = merkle_root(hashes);

        assert_eq!(root, from_hex(ROOT));
    }
}
