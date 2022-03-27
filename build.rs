use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

include!("src/rules.rs");

impl Rules {
    fn from_filename(name: &str) -> Option<Rules> {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref SUTOM_REGEX: Regex = Regex::new("^sutom-([1-9]+)([A-Z])").unwrap();
            static ref TEST_REGEX: Regex = Regex::new("^test-([1-9]+)([A-Z])").unwrap();
            static ref WORDLE_REGEX: Regex = Regex::new("^wordle").unwrap();
        }

        if let Some(capture) = SUTOM_REGEX.captures_iter(name).next() {
            return Some(Rules::SUTOM(
                capture.get(2).unwrap().as_str().chars().next().unwrap(),
                capture.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            ));
        }
        if let Some(capture) = TEST_REGEX.captures_iter(name).next() {
            return Some(Rules::TEST(
                capture.get(2).unwrap().as_str().chars().next().unwrap(),
                capture.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            ));
        }
        if WORDLE_REGEX.captures_iter(name).next().is_some() {
            return Some(Rules::WORDLE);
        }
        None
    }
}

fn build_dict_registry() {
    // Initialize empty registry
    let mut registry = DictRegistry::default();

    // Loop over each file in data directory
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&src_dir).join("data");
    let data_entries = fs::read_dir(data_dir).unwrap();
    data_entries
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().is_ok())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .for_each(|entry| {
            if let Some(rule) = Rules::from_filename(entry.file_name().to_str().unwrap()) {
                println!("cargo:rerun-if-changed={:?}", entry.path());
                let dict = Dict::from_file(entry.path()).unwrap();
                registry.insert(rule, dict);
            }
        });

    // Write registry in file
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dict_registry.bin");

    let registry = bincode::serialize(&registry).unwrap();

    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&registry).unwrap();
    let registry = encoder.finish().unwrap();

    fs::write(dest_path, registry).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Generate dict_registry from files found in data/dict
    build_dict_registry();
}

#[test]
fn test_sutom_rule_from_filename() {
    assert_eq!(Rules::from_filename("sutom-7B.json"), Rules::SUTOM('B', 7));
}
