//! Shared conversions used by more than one `json.*` native.

use crate::vm::Value;
use serde_json::Value as Json;

/// Converts any `Value` into its JSON representation — `Json` values pass
/// through (cloning the inner document), while `Int`/`Str` are lifted into
/// the corresponding JSON scalar. Used by `json.set`/`json.push` so callers
/// can insert plain register values without an explicit `json.from_*` first.
pub(super) fn value_to_json(value: &Value) -> Json {
    match value {
        Value::Int(n) => Json::from(*n),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(Json::Number)
            .unwrap_or(Json::Null),
        Value::Str(s) => Json::from(s.as_ref()),
        Value::Json(j) => (**j).clone(),
    }
}
