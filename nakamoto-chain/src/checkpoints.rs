use crate::blocktree::Height;
use bitcoin::hash_types::BlockHash;
use bitcoin_hashes::hex::FromHex;

use std::collections::HashMap;

#[rustfmt::skip]
const CHECKPOINTS: &[(Height, &str)] = &[
    (11111,  "0000000069e244f73d78e8fd29ba2fd2ed618bd6fa2ee92559f542fdb26e7c1d"),
    (33333,  "000000002dd5588a74784eaa7ab0507a18ad16a236e7b1ce69f00d7ddfb5d0a6"),
    (74000,  "0000000000573993a3c9e41ce34471c079dcf5f52a0e824a81e7f953b8661a20"),
    (105000, "00000000000291ce28027faea320c8d2b054b2e0fe44a773f3eefb151d6bdc97"),
    (134444, "00000000000005b12ffd4cd315cd34ffd4a594f430ac814c91184a0d42d2b0fe"),
    (168000, "000000000000099e61ea72015e79632f216fe6cb33d7899acb35b75c8303b763"),
    (193000, "000000000000059f452a5f7340de6682a977387c17010ff6e6c3bd83ca8b1317"),
    (210000, "000000000000048b95347e83192f69cf0366076336c639f9b7228e9ba171342e"),
    (216116, "00000000000001b4f4b433e81ee46494af945cf96014816a4e2370f11b23df4e"),
    (225430, "00000000000001c108384350f74090433e7fcf79a606b8e797f065b130575932"),
    (250000, "000000000000003887df1f29024b06fc2200b55f8af8f35453d7be294df2d214"),
    (279000, "0000000000000001ae8c72a0b0c301f67e3afca10e819efa9041e458e9bd7e40"),
    (295000, "00000000000000004d9b4ef50f0f9d686fd69db2e03af35a100370c64632a983"),
];

pub fn checkpoints() -> HashMap<Height, BlockHash> {
    let mut chks = HashMap::new();

    for (height, hash) in CHECKPOINTS.iter() {
        let hash = BlockHash::from_hex(hash).unwrap();
        chks.insert(*height, hash);
    }
    chks
}
