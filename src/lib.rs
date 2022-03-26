pub mod rules;

use rules::DictRegistry;
use std::io::Write;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DICT_REGISTRY: DictRegistry = {
        let compressed = include_bytes!(concat!(env!("OUT_DIR"), "/dict_registry.bin"));
        let mut decoder = flate2::write::ZlibDecoder::new(Vec::new());
        decoder.write_all(compressed).unwrap();
        let registry = decoder.finish().unwrap();
        let registry: DictRegistry = bincode::deserialize(&registry).unwrap();
        registry
    };
}

#[test]
fn test_dict_registry() {
    assert!(DICT_REGISTRY.get(&rules::Rules::TEST('A', 6)).is_some());
    assert!(DICT_REGISTRY.get(&rules::Rules::TEST('Z', 6)).is_none());
}
