use serde::Deserialize;

/// Boolean expression over named filter groups.
///
/// `Filter(name)` references a key in [`Rule::filters`](crate::rules::Rule::filters).
/// The tree is validated at parse time by [`parse_rules`](crate::rules::parse_rules).
#[derive(Debug, Deserialize)]
pub enum Condition {
    Filter(String),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    /// Checks that every [`Filter`](Condition::Filter) leaf references a name present in `filters`.
    ///
    /// Returns `Err` with the name of the first undefined filter found.
    pub(crate) fn validate(&self, filters: &[String]) -> Result<(), String> {
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
