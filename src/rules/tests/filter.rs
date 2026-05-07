use crate::rules::filter::{FieldFilter, FilterTypes, Filters};

fn load(name: &str) -> Filters {
    let content = std::fs::read_to_string(format!("tests/fixtures/filters/{name}")).unwrap();
    serde_yaml::from_str(&content).unwrap()
}

fn parse(yaml: &str) -> Filters {
    serde_yaml::from_str(yaml).unwrap()
}

fn find(filters: Filters, field: &str) -> FieldFilter {
    filters.0.into_iter().find(|ff| ff.field == field).unwrap()
}

#[test]
fn exact_is_default_when_no_operator() {
    assert!(matches!(find(load("process_filter.yaml"), "hostname").condition, FilterTypes::Exact));
}

#[test]
fn contains_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "proc_name").condition, FilterTypes::Contains));
}

#[test]
fn startswith_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "cmd_line").condition, FilterTypes::Startswith));
}

#[test]
fn endswith_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "binary").condition, FilterTypes::Endswith));
}

#[test]
fn gt_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "pid").condition, FilterTypes::Gt));
}

#[test]
fn gte_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "min_pid").condition, FilterTypes::Gte));
}

#[test]
fn lt_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "nice_val").condition, FilterTypes::Lt));
}

#[test]
fn lte_operator_parsed_correctly() {
    assert!(matches!(find(load("process_filter.yaml"), "max_nice").condition, FilterTypes::Lte));
}

#[test]
fn unknown_operator_is_dropped() {
    assert!(parse(r#"tool|unsupported_op: !String "nmap""#).0.is_empty());
}

#[test]
fn heterogeneous_types_are_dropped() {
    assert!(parse("field|exact:\n  - !String \"text\"\n  - !Integer 42").0.is_empty());
}

#[test]
fn scalar_value_is_wrapped_in_vec() {
    assert_eq!(find(load("process_filter.yaml"), "cmd_line").values.len(), 1);
}

#[test]
fn multiple_values_all_stored() {
    assert_eq!(find(load("process_filter.yaml"), "proc_name").values.len(), 2);
}

#[test]
fn field_name_extracted_correctly() {
    assert!(load("process_filter.yaml").0.iter().any(|ff| ff.field == "proc_name"));
}
