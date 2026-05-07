use std::path::Path;

use crate::rules::{Rule, parse_rules};

const RULE_MISSING_FIELDS: &str = r#"id: "rule-002"
title: "No Matcher"
level: low"#;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/rules/{name}")).unwrap()
}

#[test]
fn valid_rule_parses_correctly() {
    let rule: Rule = serde_yaml::from_str(&fixture("valid_rule.yaml")).unwrap();
    assert_eq!(rule.id, "rule-001");
}

#[test]
fn invalid_rule_missing_field_returns_deserialization_error() {
    assert!(serde_yaml::from_str::<Rule>(RULE_MISSING_FIELDS).is_err());
}

#[test]
fn invalid_rule_undefined_condition_fails_validation() {
    let rule: Rule = serde_yaml::from_str(&fixture("invalid_undefined_condition.yaml")).unwrap();
    let filter_names: Vec<String> = rule.filters.keys().cloned().collect();
    assert!(rule.condition.validate(&filter_names).is_err());
}

#[test]
fn parse_rules_skips_invalid_returns_valid() {
    let rules = parse_rules(Path::new("tests/fixtures/rules")).unwrap();
    assert_eq!(rules.len(), 1);
}
