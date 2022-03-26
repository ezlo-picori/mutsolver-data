use mutsolver_data::{rules, DICT_REGISTRY};

#[test]
fn test_dict_registry() {
    assert!(DICT_REGISTRY.get(&rules::Rules::TEST('A', 6)).is_some());
    assert!(DICT_REGISTRY.get(&rules::Rules::TEST('Z', 6)).is_none());
}
