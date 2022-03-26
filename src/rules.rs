/// Structure defining available game rules
use mutsolver_core::Dict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, Debug, Deserialize, Serialize)]
pub enum Rules {
    SUTOM(char, u8), // First letter, word size
    TEST(char, u8),  // First letter, word size
}

pub type DictRegistry = HashMap<Rules, Dict>;
