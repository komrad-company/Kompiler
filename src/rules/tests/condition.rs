use crate::rules::condition::Condition;

fn filter_names(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

#[test]
fn filter_valid_returns_ok() {
    assert!(Condition::Filter("a".into()).validate(&filter_names(&["a"])).is_ok());
}

#[test]
fn filter_undefined_returns_err() {
    assert!(Condition::Filter("x".into()).validate(&filter_names(&["a"])).is_err());
}

#[test]
fn and_both_valid_returns_ok() {
    let cond = Condition::And(
        Box::new(Condition::Filter("a".into())),
        Box::new(Condition::Filter("b".into())),
    );
    assert!(cond.validate(&filter_names(&["a", "b"])).is_ok());
}

#[test]
fn and_left_undefined_returns_err() {
    let cond = Condition::And(
        Box::new(Condition::Filter("x".into())),
        Box::new(Condition::Filter("b".into())),
    );
    assert!(cond.validate(&filter_names(&["b"])).is_err());
}

#[test]
fn and_right_undefined_returns_err() {
    let cond = Condition::And(
        Box::new(Condition::Filter("a".into())),
        Box::new(Condition::Filter("x".into())),
    );
    assert!(cond.validate(&filter_names(&["a"])).is_err());
}

#[test]
fn or_both_valid_returns_ok() {
    let cond = Condition::Or(
        Box::new(Condition::Filter("a".into())),
        Box::new(Condition::Filter("b".into())),
    );
    assert!(cond.validate(&filter_names(&["a", "b"])).is_ok());
}

#[test]
fn or_one_undefined_returns_err() {
    let cond = Condition::Or(
        Box::new(Condition::Filter("a".into())),
        Box::new(Condition::Filter("x".into())),
    );
    assert!(cond.validate(&filter_names(&["a"])).is_err());
}

#[test]
fn not_valid_returns_ok() {
    let cond = Condition::Not(Box::new(Condition::Filter("a".into())));
    assert!(cond.validate(&filter_names(&["a"])).is_ok());
}

#[test]
fn not_undefined_returns_err() {
    let cond = Condition::Not(Box::new(Condition::Filter("x".into())));
    assert!(cond.validate(&filter_names(&["a"])).is_err());
}

#[test]
fn nested_undefined_deep_leaf_returns_err() {
    let cond = Condition::And(
        Box::new(Condition::Or(
            Box::new(Condition::Filter("a".into())),
            Box::new(Condition::Filter("b".into())),
        )),
        Box::new(Condition::Not(Box::new(Condition::Filter("x".into())))),
    );
    assert!(cond.validate(&filter_names(&["a", "b"])).is_err());
}
