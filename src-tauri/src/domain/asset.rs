use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::PengError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    Draft,
    Ready,
    Deprecated,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    Created,
    Imported,
    Linked,
    Generated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetSource {
    pub kind: SourceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl Default for AssetSource {
    fn default() -> Self {
        Self {
            kind: SourceKind::Created,
            reference: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub schema_version: String,
    pub name: String,
    pub summary: String,
    pub status: AssetStatus,
    pub tags: Vec<String>,
    pub body: String,
    pub type_data: Value,
    pub source: AssetSource,
    pub current_revision: i64,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub unknown: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetRequest {
    #[serde(rename = "type")]
    pub asset_type: String,
    pub schema_version: String,
    pub name: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default = "draft_status")]
    pub status: AssetStatus,
    #[serde(default)]
    pub tags: Vec<String>,
    pub body: String,
    #[serde(default = "empty_object")]
    pub type_data: Value,
    #[serde(default)]
    pub source: AssetSource,
    #[serde(flatten)]
    pub unknown: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssetRequest {
    pub id: String,
    pub expected_revision: i64,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub schema_version: String,
    pub name: String,
    pub summary: String,
    pub status: AssetStatus,
    pub tags: Vec<String>,
    pub body: String,
    pub type_data: Value,
    pub source: AssetSource,
    #[serde(flatten)]
    pub unknown: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAssetRequest {
    pub id: String,
    pub expected_revision: i64,
}

fn draft_status() -> AssetStatus {
    AssetStatus::Draft
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}

pub fn validate_create(request: &CreateAssetRequest) -> Result<(), PengError> {
    validate_fields(
        &request.asset_type,
        &request.schema_version,
        &request.name,
        &request.summary,
        &request.tags,
        &request.body,
        &request.type_data,
    )
}

pub fn validate_update(request: &UpdateAssetRequest) -> Result<(), PengError> {
    validate_uuid(&request.id)?;
    validate_revision(request.expected_revision)?;
    validate_fields(
        &request.asset_type,
        &request.schema_version,
        &request.name,
        &request.summary,
        &request.tags,
        &request.body,
        &request.type_data,
    )
}

pub fn validate_delete(request: &DeleteAssetRequest) -> Result<(), PengError> {
    validate_uuid(&request.id)?;
    validate_revision(request.expected_revision)
}

pub fn validate_uuid(value: &str) -> Result<(), PengError> {
    let valid = value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        });
    if valid {
        Ok(())
    } else {
        Err(PengError::validation("id", "ID must be a canonical UUID."))
    }
}

fn validate_revision(value: i64) -> Result<(), PengError> {
    if value >= 1 {
        Ok(())
    } else {
        Err(PengError::validation(
            "expectedRevision",
            "Expected revision must be at least 1.",
        ))
    }
}

fn validate_fields(
    asset_type: &str,
    schema_version: &str,
    name: &str,
    summary: &str,
    tags: &[String],
    body: &str,
    type_data: &Value,
) -> Result<(), PengError> {
    let valid_type = (1..=64).contains(&asset_type.len())
        && asset_type.as_bytes()[0].is_ascii_lowercase()
        && asset_type
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b));
    if !valid_type {
        return Err(PengError::validation(
            "type",
            "Type must be a lowercase asset token of at most 64 characters.",
        ));
    }
    let schema_parts: Vec<&str> = schema_version.split('.').collect();
    if schema_parts.is_empty()
        || schema_parts
            .iter()
            .any(|part| part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err(PengError::validation(
            "schemaVersion",
            "Schema version must contain dot-separated decimal numbers.",
        ));
    }
    let name_len = name.chars().count();
    if name.trim().is_empty() || name_len > 300 {
        return Err(PengError::validation(
            "name",
            "Name must contain 1 to 300 characters.",
        ));
    }
    if summary.chars().count() > 2000 {
        return Err(PengError::validation(
            "summary",
            "Summary must contain at most 2000 characters.",
        ));
    }
    if body.trim().is_empty() {
        return Err(PengError::validation("body", "Body is required."));
    }
    let mut seen = HashSet::new();
    for tag in tags {
        let length = tag.chars().count();
        if tag.trim().is_empty() || length > 100 {
            return Err(PengError::validation(
                "tags",
                "Each tag must contain 1 to 100 characters.",
            ));
        }
        if !seen.insert(tag) {
            return Err(PengError::validation("tags", "Tags must be unique."));
        }
    }
    if !type_data.is_object() {
        return Err(PengError::validation(
            "typeData",
            "Type data must be a JSON object.",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> CreateAssetRequest {
        CreateAssetRequest {
            asset_type: "prompt".into(),
            schema_version: "1.0".into(),
            name: "Summarise evidence".into(),
            summary: "A grounded summary prompt".into(),
            status: AssetStatus::Draft,
            tags: vec!["analysis".into()],
            body: "Summarise the supplied evidence.".into(),
            type_data: serde_json::json!({ "task": "summarisation" }),
            source: AssetSource::default(),
            unknown: Map::new(),
        }
    }

    #[test]
    fn valid_unknown_type_and_properties_round_trip() {
        let mut request = valid_request();
        request.asset_type = "future.asset".into();
        request
            .unknown
            .insert("futureField".into(), serde_json::json!({"kept": true}));
        validate_create(&request).expect("valid open type");
        let value = serde_json::to_value(&request).expect("serialize request");
        let decoded: CreateAssetRequest =
            serde_json::from_value(value).expect("deserialize request");
        assert_eq!(decoded.asset_type, "future.asset");
        assert_eq!(decoded.unknown, request.unknown);
    }

    #[test]
    fn rejects_every_named_invalid_boundary() {
        let cases = [
            ("type", "Prompt", "type"),
            ("schema", "1..0", "schemaVersion"),
            ("name", " ", "name"),
            ("body", " ", "body"),
        ];
        for (case, value, expected_field) in cases {
            let mut request = valid_request();
            match case {
                "type" => request.asset_type = value.into(),
                "schema" => request.schema_version = value.into(),
                "name" => request.name = value.into(),
                "body" => request.body = value.into(),
                _ => unreachable!(),
            }
            assert_eq!(
                validate_create(&request).unwrap_err().field.as_deref(),
                Some(expected_field)
            );
        }
        let mut request = valid_request();
        request.summary = "x".repeat(2001);
        assert_eq!(
            validate_create(&request).unwrap_err().field.as_deref(),
            Some("summary")
        );
        let mut request = valid_request();
        request.tags = vec!["same".into(), "same".into()];
        assert_eq!(
            validate_create(&request).unwrap_err().field.as_deref(),
            Some("tags")
        );
        let mut request = valid_request();
        request.tags = vec!["".into()];
        assert_eq!(
            validate_create(&request).unwrap_err().field.as_deref(),
            Some("tags")
        );
        let mut request = valid_request();
        request.type_data = Value::Null;
        assert_eq!(
            validate_create(&request).unwrap_err().field.as_deref(),
            Some("typeData")
        );
        assert!(validate_uuid("not-a-uuid").is_err());
        assert!(validate_revision(0).is_err());
    }
}
