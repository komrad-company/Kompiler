# Kompiler

![CI](https://github.com/komrad-company/Kompiler/actions/workflows/ci.yml/badge.svg) ![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue)

> *"A rule not parsed is a threat not defeated."*
> — Komrad Engineering Collective, May 2026

Kompiler is the official rule parsing library of the Komrad security collective. It transforms YAML detection rules into typed Rust structs, ready to be processed by [Korelator](https://github.com/komrad-company/Korelator), the correlation engine of the people.

Kompiler does not evaluate. Kompiler does not correlate. Kompiler **parses**. Each crate serves the collective with discipline and a single purpose.

```
YAML files  ──parse_rules()──►  Vec<Rule>  ──► Korelator (evaluation by the collective)
```

---

## Rule format

A rule is a YAML file. Every field serves the collective. No field is optional without reason.

| Field | Type | Required | Purpose |
|---|---|---|---|
| `id` | string | ✓ | Unique identifier — no rule goes unnamed |
| `title` | string | ✓ | Human-readable name for the registry |
| `level` | string | ✓ | Threat severity, as decreed: `informational` / `low` / `medium` / `high` / `critical` |
| `description` | string | | Optional clarification for future comrades |
| `tags` | list of strings | | Taxonomy tags (e.g. MITRE ATT&CK technique IDs) |
| `matcher` | mapping | ✓ | The strategy by which events are judged |
| `filters` | mapping | ✓ | Named groups of field comparisons |
| `condition` | expression | ✓ | Boolean expression over filter group names |

### Matcher

The matcher determines when the collective is alerted.

**Single** — every suspicious event is reported without delay:

```yaml
matcher: Single
```

**Threshold** — the collective is alerted only when the enemy reaches a defined threshold within a time window:

```yaml
matcher:
  !Threshold
    timeframe_secs: 60
    aggregate:
      !Count 10
    group_by:
      - user_id
```

### Filters

Filter groups are the eyes of the collective. Each group is named and contains field comparisons using the `field|operator: value` syntax.

```yaml
filters:
  process:
    - process_name|contains:
        - !String "shell"
        - !String "bash"
    - pid|gt: !Integer 1000
  user:
    - username|exact: !String "root"
    - uid|gte: !Integer 0
```

**Approved operators:**

| Operator | Verdict |
|---|---|
| *(none)* or `exact` | Exact match — no deviation tolerated |
| `contains` | Substring match |
| `startswith` | Prefix match |
| `endswith` | Suffix match |
| `gt` | Strictly greater than |
| `gte` | Greater than or equal |
| `lt` | Strictly less than |
| `lte` | Less than or equal |

**Value types — three are permitted, no more:**

```yaml
!String "text"
!Integer 42
!Boolean true
```

Multiple values in a single comparison are evaluated as a logical **OR** — any one match is sufficient for suspicion. All values in a comparison must share the same type. Heterogeneous lists are rejected and discarded with a warning. The collective does not tolerate inconsistency.

### Condition

A boolean expression that combines filter group names. The condition determines which groups must match for an event to be flagged.

```yaml
condition: !Filter process                # a single group stands accused

condition: !And
  - !Filter process
  - !Filter user                          # both must be guilty

condition: !Or
  - !Filter process
  - !Filter user                          # one is enough

condition: !Not
  !Filter process                         # innocence by exclusion
```

Conditions may be nested to any depth. Every name referenced in the condition must exist as a key in `filters`. Rules that reference an undefined filter are discarded at parse time. The collective does not load broken rules.

### Complete example

```yaml
id: "rule-001"
title: "Suspicious shell spawned by admin account"
level: high
tags:
  - "T1059"

matcher:
  !Threshold
    timeframe_secs: 60
    aggregate:
      !Count 10
    group_by: []

filters:
  process:
    - process_name|contains:
        - !String "shell"
        - !String "sh"
  user:
    - username|contains: !String "admin"
    - account|startswith: !String "adm"
    - id: !Integer 1

condition: !Or
  - !Filter process
  - !Filter user
```

---

## API

The collective exposes one function. One is sufficient.

```rust
use kompiler::{parse_rules, Rule, UnforgivableErrors};
use std::path::Path;

fn main() -> Result<(), UnforgivableErrors> {
    let rules: Vec<Rule> = parse_rules(Path::new("./rules"))?;
    Ok(())
}
```

`parse_rules` returns `Err(UnforgivableErrors)` only when the rules directory itself cannot be read — a failure of infrastructure, not of rules. Individual files that fail to parse or fail condition validation are skipped with a warning logged via [Khronika](https://github.com/komrad-company/Khronika). The collective continues. One bad file does not stop the work.

### Public types

| Type | Role in the collective |
|---|---|
| `Rule` | A fully parsed and validated detection rule |
| `RuleLevel` | Severity level — from `Informational` to `Critical` |
| `Matcher` | Matching strategy (`Single` or `Threshold`) |
| `AggregationType` | Aggregation function (`Count`) |
| `Condition` | Boolean expression tree over filter groups |
| `Filters` | Named group of `FieldFilter`s |
| `FieldFilter` | Single field comparison |
| `FilterTypes` | Comparison operator |
| `Types` | Typed value (`Boolean`, `String`, `Integer`) |
| `UnforgivableErrors` | Fatal errors — the caller must handle them or face consequences |

---

## Dependencies

Each dependency was evaluated by the collective before admission. None were added lightly.

| Crate | Purpose |
|---|---|
| `serde` + `serde_yaml` | YAML deserialization |
| `thiserror` | Error type derivation |
| `khronika` | Structured logging — our own, built by the collective |

---

## License

AGPL-3.0-or-later — the source remains open, as all things should be.
