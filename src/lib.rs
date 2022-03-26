pub mod rules;

use rules::DictRegistry;
use std::io::Write;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref DICT_REGISTRY: DictRegistry = {
        let compressed = include_bytes!(concat!(env!("OUT_DIR"), "/dict_registry.bin"));
        let mut decoder = flate2::write::ZlibDecoder::new(Vec::new());
        decoder.write_all(compressed).unwrap();
        let registry = decoder.finish().unwrap();
        let registry: DictRegistry = bincode::deserialize(&registry).unwrap();
        registry
    };
}
