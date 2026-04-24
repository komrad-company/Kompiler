use std::{fs, path::Path};

use crate::rules::parse_rules;

const STATIC_RULES: &str = "./examples/rules";
#[test]
fn parse_rules_return_parsed_rules() {
    // Ensure static rules contains necessary files
    let path = Path::new(STATIC_RULES);
    assert!(path.is_dir());
    assert_eq!(fs::read_dir(path).unwrap().count(), 2);

    let rules = parse_rules(Path::new(STATIC_RULES));
    match rules {
        Ok(v) => assert_eq!(v.len(), 1),
        Err(e) => panic!("Critical error: {:?}", e),
    }
}
