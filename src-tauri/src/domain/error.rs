use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PengError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    pub retryable: bool,
}

impl PengError {
    pub fn validation(field: &str, message: impl Into<String>) -> Self {
        Self {
            code: "VALIDATION_ERROR".into(),
            message: message.into(),
            field: Some(field.into()),
            details: None,
            retryable: false,
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            code: "NOT_FOUND".into(),
            message: format!("Asset {id} was not found."),
            field: Some("id".into()),
            details: Some(serde_json::json!({ "id": id })),
            retryable: false,
        }
    }

    pub fn conflict(id: &str, expected_revision: i64) -> Self {
        Self {
            code: "CONFLICT".into(),
            message: format!("Asset {id} changed since revision {expected_revision}."),
            field: Some("expectedRevision".into()),
            details: Some(serde_json::json!({
                "id": id,
                "expectedRevision": expected_revision
            })),
            retryable: true,
        }
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self {
            code: "STORAGE_ERROR".into(),
            message: message.into(),
            field: None,
            details: None,
            retryable: true,
        }
    }

    pub fn initialization(message: impl Into<String>) -> Self {
        Self {
            code: "INITIALIZATION_ERROR".into(),
            message: message.into(),
            field: None,
            details: None,
            retryable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_serialization_has_the_stable_shape() {
        let value = serde_json::to_value(PengError::validation("name", "Name is required."))
            .expect("error serializes");
        assert_eq!(
            value,
            serde_json::json!({
                "code": "VALIDATION_ERROR",
                "message": "Name is required.",
                "field": "name",
                "retryable": false
            })
        );
    }
}
