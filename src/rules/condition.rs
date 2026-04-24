use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Condition {
    Filter(String),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    pub fn validate(&self, filters: &Vec<String>) -> Result<(), String> {
        match self {
            Condition::Filter(str) => filters
                .contains(str)
                .then_some(())
                .ok_or_else(|| format!("Undefined Filter `{}`", str)),
            Condition::And(left, right) | Condition::Or(left, right) => {
                left.validate(filters)?;
                right.validate(filters)
            }
            Condition::Not(cond) => cond.validate(filters),
        }
    }
}
