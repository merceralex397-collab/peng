use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use crate::domain::asset::{AssetStatus, CreateAssetRequest, UpdateAssetRequest};
use crate::domain::{Asset, PengError};
use rusqlite::{Connection, OptionalExtension, Row, params};

#[derive(Debug)]
pub struct AssetStore {
    connection: Mutex<Connection>,
}

struct StoredAsset {
    id: String,
    asset_type: String,
    schema_version: String,
    name: String,
    summary: String,
    status: String,
    tags_json: String,
    body: String,
    type_data_json: String,
    source_json: String,
    unknown_json: String,
    current_revision: i64,
    created_at: String,
    updated_at: String,
}

impl AssetStore {
    pub fn open(path: &Path) -> Result<Self, PengError> {
        let connection = Connection::open(path).map_err(|error| {
            PengError::initialization(format!("Could not open Peng database: {error}"))
        })?;
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS assets (
                    id TEXT PRIMARY KEY NOT NULL,
                    asset_type TEXT NOT NULL,
                    schema_version TEXT NOT NULL,
                    name TEXT NOT NULL,
                    summary TEXT NOT NULL,
                    status TEXT NOT NULL,
                    tags_json TEXT NOT NULL,
                    body TEXT NOT NULL,
                    type_data_json TEXT NOT NULL,
                    source_json TEXT NOT NULL,
                    unknown_json TEXT NOT NULL,
                    current_revision INTEGER NOT NULL CHECK (current_revision >= 1),
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );",
            )
            .map_err(|error| {
                PengError::initialization(format!("Could not initialize Peng database: {error}"))
            })?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn generate_uuid_v4(&self) -> Result<String, PengError> {
        let connection = self.lock()?;
        let mut bytes: Vec<u8> = connection
            .query_row("SELECT randomblob(16)", [], |row| row.get(0))
            .map_err(storage_error)?;
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Ok(format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15]
        ))
    }

    pub fn create(&self, id: &str, request: &CreateAssetRequest) -> Result<Asset, PengError> {
        let mut connection = self.lock()?;
        let transaction = connection.transaction().map_err(storage_error)?;
        let tags = json(&request.tags)?;
        let type_data = json(&request.type_data)?;
        let source = json(&request.source)?;
        let unknown = json(&request.unknown)?;
        transaction
            .execute(
                "INSERT INTO assets (
                    id, asset_type, schema_version, name, summary, status, tags_json, body,
                    type_data_json, source_json, unknown_json, current_revision, created_at, updated_at
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 1,
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                )",
                params![
                    id,
                    request.asset_type,
                    request.schema_version,
                    request.name,
                    request.summary,
                    status_name(&request.status),
                    tags,
                    request.body,
                    type_data,
                    source,
                    unknown,
                ],
            )
            .map_err(|error| match error {
                rusqlite::Error::SqliteFailure(ref failure, _)
                    if failure.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    PengError::conflict(id, 0)
                }
                other => storage_error(other),
            })?;
        let asset = query_asset(&transaction, id)?.ok_or_else(|| {
            PengError::storage("Created asset could not be read from the database.")
        })?;
        transaction.commit().map_err(storage_error)?;
        Ok(asset)
    }

    pub fn get(&self, id: &str) -> Result<Asset, PengError> {
        let connection = self.lock()?;
        query_asset(&connection, id)?.ok_or_else(|| PengError::not_found(id))
    }

    pub fn update(&self, request: &UpdateAssetRequest) -> Result<Asset, PengError> {
        let mut connection = self.lock()?;
        let transaction = connection.transaction().map_err(storage_error)?;
        let changed = transaction
            .execute(
                "UPDATE assets SET
                    asset_type = ?1, schema_version = ?2, name = ?3, summary = ?4,
                    status = ?5, tags_json = ?6, body = ?7, type_data_json = ?8,
                    source_json = ?9, unknown_json = ?10,
                    current_revision = current_revision + 1,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                 WHERE id = ?11 AND current_revision = ?12",
                params![
                    request.asset_type,
                    request.schema_version,
                    request.name,
                    request.summary,
                    status_name(&request.status),
                    json(&request.tags)?,
                    request.body,
                    json(&request.type_data)?,
                    json(&request.source)?,
                    json(&request.unknown)?,
                    request.id,
                    request.expected_revision,
                ],
            )
            .map_err(storage_error)?;
        if changed == 0 {
            return Err(missing_or_conflict(
                &transaction,
                &request.id,
                request.expected_revision,
            )?);
        }
        let asset = query_asset(&transaction, &request.id)?.ok_or_else(|| {
            PengError::storage("Updated asset could not be read from the database.")
        })?;
        transaction.commit().map_err(storage_error)?;
        Ok(asset)
    }

    pub fn delete(&self, id: &str, expected_revision: i64) -> Result<(), PengError> {
        let mut connection = self.lock()?;
        let transaction = connection.transaction().map_err(storage_error)?;
        let changed = transaction
            .execute(
                "DELETE FROM assets WHERE id = ?1 AND current_revision = ?2",
                params![id, expected_revision],
            )
            .map_err(storage_error)?;
        if changed == 0 {
            return Err(missing_or_conflict(&transaction, id, expected_revision)?);
        }
        transaction.commit().map_err(storage_error)
    }

    fn lock(&self) -> Result<MutexGuard<'_, Connection>, PengError> {
        self.connection
            .lock()
            .map_err(|_| PengError::storage("The database lock is poisoned."))
    }
}

fn query_asset(connection: &Connection, id: &str) -> Result<Option<Asset>, PengError> {
    let stored = connection
        .query_row(
            "SELECT id, asset_type, schema_version, name, summary, status, tags_json, body,
                    type_data_json, source_json, unknown_json, current_revision, created_at, updated_at
             FROM assets WHERE id = ?1",
            [id],
            stored_from_row,
        )
        .optional()
        .map_err(storage_error)?;
    stored.map(decode_asset).transpose()
}

fn stored_from_row(row: &Row<'_>) -> rusqlite::Result<StoredAsset> {
    Ok(StoredAsset {
        id: row.get(0)?,
        asset_type: row.get(1)?,
        schema_version: row.get(2)?,
        name: row.get(3)?,
        summary: row.get(4)?,
        status: row.get(5)?,
        tags_json: row.get(6)?,
        body: row.get(7)?,
        type_data_json: row.get(8)?,
        source_json: row.get(9)?,
        unknown_json: row.get(10)?,
        current_revision: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

fn decode_asset(stored: StoredAsset) -> Result<Asset, PengError> {
    let status = match stored.status.as_str() {
        "draft" => AssetStatus::Draft,
        "ready" => AssetStatus::Ready,
        "deprecated" => AssetStatus::Deprecated,
        "archived" => AssetStatus::Archived,
        value => {
            return Err(PengError::storage(format!(
                "Stored status is invalid: {value}"
            )));
        }
    };
    Ok(Asset {
        id: stored.id,
        asset_type: stored.asset_type,
        schema_version: stored.schema_version,
        name: stored.name,
        summary: stored.summary,
        status,
        tags: parse_json(&stored.tags_json, "tags")?,
        body: stored.body,
        type_data: parse_json(&stored.type_data_json, "type data")?,
        source: parse_json(&stored.source_json, "source")?,
        unknown: parse_json(&stored.unknown_json, "unknown properties")?,
        current_revision: stored.current_revision,
        created_at: stored.created_at,
        updated_at: stored.updated_at,
    })
}

fn missing_or_conflict(
    connection: &Connection,
    id: &str,
    expected_revision: i64,
) -> Result<PengError, PengError> {
    let exists: bool = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM assets WHERE id = ?1)",
            [id],
            |row| row.get(0),
        )
        .map_err(storage_error)?;
    Ok(if exists {
        PengError::conflict(id, expected_revision)
    } else {
        PengError::not_found(id)
    })
}

fn json(value: &impl serde::Serialize) -> Result<String, PengError> {
    serde_json::to_string(value)
        .map_err(|error| PengError::storage(format!("Could not serialize asset data: {error}")))
}

fn parse_json<T: serde::de::DeserializeOwned>(text: &str, field: &str) -> Result<T, PengError> {
    serde_json::from_str(text)
        .map_err(|error| PengError::storage(format!("Stored {field} JSON is invalid: {error}")))
}

fn status_name(status: &AssetStatus) -> &'static str {
    match status {
        AssetStatus::Draft => "draft",
        AssetStatus::Ready => "ready",
        AssetStatus::Deprecated => "deprecated",
        AssetStatus::Archived => "archived",
    }
}

fn storage_error(error: rusqlite::Error) -> PengError {
    PengError::storage(format!("SQLite operation failed: {error}"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::domain::asset::{AssetSource, SourceKind};
    use serde_json::{Map, Value};

    fn path(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "peng-{name}-{}-{nonce}.sqlite3",
            std::process::id()
        ))
    }

    fn request() -> CreateAssetRequest {
        CreateAssetRequest {
            asset_type: "prompt".into(),
            schema_version: "1.0".into(),
            name: "Evidence summary".into(),
            summary: "Summarises supplied evidence".into(),
            status: AssetStatus::Draft,
            tags: vec!["analysis".into()],
            body: "Summarise the supplied evidence.".into(),
            type_data: serde_json::json!({"task": "summarisation"}),
            source: AssetSource {
                kind: SourceKind::Created,
                reference: None,
            },
            unknown: Map::from_iter([("futureField".into(), Value::Bool(true))]),
        }
    }

    #[test]
    fn file_backed_crud_reopens_and_preserves_data() {
        let database = path("crud");
        let id;
        {
            let store = AssetStore::open(&database).expect("open store");
            id = store.generate_uuid_v4().expect("generate id");
            let created = store.create(&id, &request()).expect("create");
            assert_eq!(created.current_revision, 1);
            assert_eq!(created.unknown["futureField"], true);
        }
        {
            let store = AssetStore::open(&database).expect("reopen store");
            let created = store.get(&id).expect("read after reopen");
            let update = UpdateAssetRequest {
                id: id.clone(),
                expected_revision: created.current_revision,
                asset_type: created.asset_type,
                schema_version: created.schema_version,
                name: "Updated evidence summary".into(),
                summary: created.summary,
                status: AssetStatus::Ready,
                tags: created.tags,
                body: created.body,
                type_data: created.type_data,
                source: created.source,
                unknown: created.unknown,
            };
            let updated = store.update(&update).expect("update");
            assert_eq!(updated.current_revision, 2);
            assert_eq!(updated.name, "Updated evidence summary");
            assert_eq!(store.update(&update).unwrap_err().code, "CONFLICT");
            assert_eq!(store.delete(&id, 1).unwrap_err().code, "CONFLICT");
            store.delete(&id, 2).expect("delete");
            assert_eq!(store.get(&id).unwrap_err().code, "NOT_FOUND");
        }
        fs::remove_file(&database).expect("remove exact temporary database");
    }

    #[test]
    fn duplicate_and_malformed_rows_surface_structured_errors() {
        let database = path("failures");
        let store = AssetStore::open(&database).expect("open store");
        let id = store.generate_uuid_v4().expect("generate id");
        store.create(&id, &request()).expect("create");
        assert_eq!(store.create(&id, &request()).unwrap_err().code, "CONFLICT");
        {
            let connection = store.connection.lock().expect("lock database");
            connection
                .execute(
                    "UPDATE assets SET tags_json = 'not-json' WHERE id = ?1",
                    [&id],
                )
                .expect("corrupt test row");
        }
        assert_eq!(store.get(&id).unwrap_err().code, "STORAGE_ERROR");
        drop(store);
        fs::remove_file(&database).expect("remove exact temporary database");
    }

    #[test]
    fn failed_open_and_bootstrap_are_initialization_errors() {
        let directory = std::env::temp_dir();
        assert_eq!(
            AssetStore::open(&directory).unwrap_err().code,
            "INITIALIZATION_ERROR"
        );
    }
}
