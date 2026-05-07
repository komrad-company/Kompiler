use crate::rules::matcher::{AggregationType, Matcher};

fn load(name: &str) -> Matcher {
    let content = std::fs::read_to_string(format!("tests/fixtures/matchers/{name}")).unwrap();
    serde_yaml::from_str(&content).unwrap()
}

#[test]
fn single_deserializes_correctly() {
    assert!(matches!(load("single.yaml"), Matcher::Single));
}

#[test]
fn threshold_timeframe_and_group_by_parsed_correctly() {
    match load("threshold_with_groups.yaml") {
        Matcher::Threshold {
            timeframe_secs,
            group_by,
            ..
        } => {
            assert_eq!(timeframe_secs, 60);
            assert_eq!(group_by, vec!["user_id"]);
        }
        _ => panic!("Expected Threshold"),
    }
}

#[test]
fn threshold_count_value_parsed_correctly() {
    match load("threshold_with_groups.yaml") {
        Matcher::Threshold {
            aggregate: AggregationType::Count(n),
            ..
        } => assert_eq!(n, 10),
        _ => panic!("Expected Threshold with Count"),
    }
}

#[test]
fn threshold_empty_group_by_parsed_correctly() {
    match load("threshold_no_groups.yaml") {
        Matcher::Threshold { group_by, .. } => assert!(group_by.is_empty()),
        _ => panic!("Expected Threshold"),
    }
}
