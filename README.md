# Kompiler

Kompiler is a Rust library that parses YAML detection rules into typed Rust structs. It is part of the [Komrad](https://github.com/komrad-company) security ecosystem and is consumed by [Korelator](https://github.com/komrad-company/Korelator) to drive event correlation.

## Overview

A rule describes **what to detect** and **how to match**. Kompiler handles the parsing and structural validation — the evaluation logic lives in the consumer.

```
YAML files  ──parse_rules()──►  Vec<Rule>  ──► Korelator (evaluation)
```

## Rule format

A rule is a YAML file with the following fields:

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | ✓ | Unique identifier |
| `title` | string | ✓ | Human-readable name |
| `level` | string | ✓ | Severity: `informational` / `low` / `medium` / `high` / `critical` |
| `description` | string | | Free-text description |
| `tags` | list of strings | | Taxonomy tags (e.g. MITRE ATT&CK technique IDs) |
| `matcher` | mapping | ✓ | Matching strategy |
| `filters` | mapping | ✓ | Named groups of field comparisons |
| `condition` | expression | ✓ | Boolean expression over filter group names |

### Matcher

**Single** — triggers on every event that satisfies the condition:

```yaml
matcher: Single
```

**Threshold** — triggers when the aggregation count is reached within the time window:

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

A filter group is a named list of field comparisons. Each comparison uses the `field|operator: value` syntax.

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

**Available operators:**

| Operator | Meaning |
|---|---|
| *(none)* or `exact` | Equality |
| `contains` | Substring match |
| `startswith` | Prefix match |
| `endswith` | Suffix match |
| `gt` | Strictly greater than |
| `gte` | Greater than or equal |
| `lt` | Strictly less than |
| `lte` | Less than or equal |

**Value types:**

```yaml
!String "text"
!Integer 42
!Boolean true
```

Multiple values in a single comparison are evaluated as a logical **OR**. All values in a comparison must share the same type — heterogeneous lists are silently dropped with a warning.

### Condition

A boolean expression that combines filter group names:

```yaml
condition: !Filter process                        # single group

condition: !And
  - !Filter process
  - !Filter user

condition: !Or
  - !Filter process
  - !Filter user

condition: !Not
  !Filter process
```

Operators can be nested arbitrarily. Every name referenced in the condition must exist as a key in `filters` — invalid rules are discarded at parse time with a warning.

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

## API

```rust
use kompiler::{parse_rules, Rule, UnforgivableErrors};
use std::path::Path;

fn main() -> Result<(), UnforgivableErrors> {
    let rules: Vec<Rule> = parse_rules(Path::new("./rules"))?;
    Ok(())
}
```

`parse_rules` returns `Err(UnforgivableErrors)` only if the rules directory itself cannot be read. Individual files that fail to parse or fail condition validation are skipped with a warning logged via [Khronika](https://github.com/komrad-company/Khronika).

### Public types

| Type | Description |
|---|---|
| `Rule` | A fully parsed and validated detection rule |
| `RuleLevel` | Severity level enum |
| `Matcher` | Matching strategy (`Single` or `Threshold`) |
| `AggregationType` | Aggregation function (`Count`) |
| `Condition` | Boolean expression tree |
| `Filters` | Named group of `FieldFilter`s |
| `FieldFilter` | Single field comparison |
| `FilterTypes` | Comparison operator |
| `Types` | Typed value (`Boolean`, `String`, `Integer`) |
| `UnforgivableErrors` | Fatal errors returned by `parse_rules` |

## Dependencies

| Crate | Purpose |
|---|---|
| `serde` + `serde_yaml` | YAML deserialization |
| `thiserror` | Error type derivation |
| `khronika` | Structured logging |

## License

AGPL-3.0-or-later
