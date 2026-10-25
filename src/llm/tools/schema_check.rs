//! Checking tool-call arguments against a tool's JSON Schema.
//!
//! Small local models often send malformed arguments: a string where an
//! object is expected, an array instead of a path, no object at all. Running
//! the tool then fails with a terse deserialization error the model cannot
//! act on. Two helpers fix that:
//!
//! - [`check_arguments`] rejects a call before it runs, but only when it is
//!   provably wrong: arguments that are not an object, or a declared field
//!   whose JSON kind (text, number, list, object) cannot be what the schema
//!   says. It does not enforce `required` or `enum`: several tools accept
//!   alias keys the schema does not list (`file_path` for `path`), and a
//!   strict check would reject calls that work.
//! - [`describe_arguments`] renders the schema as a one-line argument list,
//!   appended to the error when a tool rejects its input, so the model sees
//!   exactly what to send.

use serde_json::Value;

/// Reject arguments that cannot match `schema`; `Ok` when they might.
pub fn check_arguments(input: &Value, schema: &Value) -> Result<(), String> {
    if schema.get("type").and_then(Value::as_str) == Some("object") && !input.is_object() {
        return Err(format!(
            "arguments must be a JSON object of named fields, got {}",
            kind_of(input)
        ));
    }

    let (Some(fields), Some(properties)) = (
        input.as_object(),
        schema.get("properties").and_then(Value::as_object),
    ) else {
        return Ok(());
    };

    for (name, value) in fields {
        let Some(expected) = properties.get(name).map(declared_types) else {
            continue;
        };
        if !expected.is_empty() && !expected.iter().any(|t| kind_fits(t, value)) {
            return Err(format!(
                "field '{}' must be {}, got {}",
                name,
                expected.join(" or "),
                kind_of(value)
            ));
        }
    }
    Ok(())
}

/// One line describing the arguments `schema` expects, for example
/// `path (string, required), offset (integer), mode ("read"|"write")`.
pub fn describe_arguments(schema: &Value) -> String {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return "no arguments".to_string();
    };
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|r| r.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    let mut fields: Vec<(&String, &Value)> = properties.iter().collect();
    // Required fields first, in their declared order, then the rest.
    fields.sort_by_key(|(name, _)| {
        required
            .iter()
            .position(|r| r == name)
            .unwrap_or(usize::MAX)
    });

    fields
        .into_iter()
        .map(|(name, property)| {
            let mut traits = Vec::new();
            if let Some(options) = property.get("enum").and_then(Value::as_array) {
                traits.push(
                    options
                        .iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<_>>()
                        .join("|"),
                );
            } else {
                let types = declared_types(property);
                if !types.is_empty() {
                    traits.push(types.join(" or "));
                }
            }
            if required.contains(&name.as_str()) {
                traits.push("required".to_string());
            }
            if traits.is_empty() {
                name.clone()
            } else {
                format!("{name} ({})", traits.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The JSON Schema `type` of a property, as a list (`"type"` may be a
/// string or an array of strings).
fn declared_types(property: &Value) -> Vec<String> {
    match property.get("type") {
        Some(Value::String(t)) => vec![t.clone()],
        Some(Value::Array(ts)) => ts
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

/// Whether `value` can be a `schema_type`. Scalars are lenient with each
/// other (a tool may parse `"10"` as a number), so only a container where a
/// scalar is expected, or the reverse, is a mismatch.
fn kind_fits(schema_type: &str, value: &Value) -> bool {
    match schema_type {
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" | "integer" | "number" | "boolean" => !(value.is_object() || value.is_array()),
        "null" => value.is_null(),
        _ => true,
    }
}

fn kind_of(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "a list",
        Value::Object(_) => "an object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "file_path": {"type": "string"},
                "offset": {"type": "integer"},
                "patterns": {"type": "array"},
                "mode": {"type": "string", "enum": ["read", "write"]}
            },
            "required": ["path"]
        })
    }

    #[test]
    fn accepts_well_formed_and_alias_arguments() {
        assert!(check_arguments(&json!({"path": "a.rs", "offset": 3}), &schema()).is_ok());
        // `required` is not enforced: the alias key works for the tool.
        assert!(check_arguments(&json!({"file_path": "a.rs"}), &schema()).is_ok());
        // Undeclared fields are left to the tool.
        assert!(check_arguments(&json!({"path": "a.rs", "extra": [1]}), &schema()).is_ok());
        // Scalars are lenient: a numeric string may be parsed by the tool.
        assert!(check_arguments(&json!({"path": "a.rs", "offset": "3"}), &schema()).is_ok());
    }

    #[test]
    fn rejects_provably_wrong_arguments() {
        let err = check_arguments(&json!("a.rs"), &schema()).unwrap_err();
        assert!(err.contains("JSON object"), "{err}");

        let err = check_arguments(&json!({"path": ["a.rs"]}), &schema()).unwrap_err();
        assert_eq!(err, "field 'path' must be string, got a list");

        let err = check_arguments(&json!({"patterns": "*.rs"}), &schema()).unwrap_err();
        assert_eq!(err, "field 'patterns' must be array, got a string");
    }

    #[test]
    fn describes_required_fields_first_with_types_and_options() {
        let line = describe_arguments(&schema());
        assert!(line.starts_with("path (string, required)"), "{line}");
        assert!(line.contains("offset (integer)"), "{line}");
        assert!(line.contains(r#"mode ("read"|"write")"#), "{line}");
    }

    #[test]
    fn describes_a_schema_without_properties() {
        assert_eq!(
            describe_arguments(&json!({"type": "object"})),
            "no arguments"
        );
    }
}
