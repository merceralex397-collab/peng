use std::path::Path;

use crate::domain::asset::{validate_create, validate_delete, validate_update, validate_uuid};
use crate::domain::{Asset, CreateAssetRequest, DeleteAssetRequest, PengError, UpdateAssetRequest};
use crate::storage::AssetStore;

pub struct AssetService {
    store: AssetStore,
}

impl AssetService {
    pub fn open(path: &Path) -> Result<Self, PengError> {
        Ok(Self {
            store: AssetStore::open(path)?,
        })
    }

    pub fn create(&self, request: CreateAssetRequest) -> Result<Asset, PengError> {
        validate_create(&request)?;
        let id = self.store.generate_uuid_v4()?;
        self.store.create(&id, &request)
    }

    pub fn get(&self, id: &str) -> Result<Asset, PengError> {
        validate_uuid(id)?;
        self.store.get(id)
    }

    pub fn update(&self, request: UpdateAssetRequest) -> Result<Asset, PengError> {
        validate_update(&request)?;
        self.store.update(&request)
    }

    pub fn delete(&self, request: DeleteAssetRequest) -> Result<(), PengError> {
        validate_delete(&request)?;
        self.store.delete(&request.id, request.expected_revision)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::{Map, Value};

    use super::*;
    use crate::domain::asset::{AssetSource, AssetStatus};

    fn path() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "peng-service-{}-{nonce}.sqlite3",
            std::process::id()
        ))
    }

    fn request() -> CreateAssetRequest {
        CreateAssetRequest {
            asset_type: "prompt".into(),
            schema_version: "1.0".into(),
            name: "Review evidence".into(),
            summary: String::new(),
            status: AssetStatus::Draft,
            tags: vec![],
            body: "Review the evidence carefully.".into(),
            type_data: Value::Object(Map::new()),
            source: AssetSource::default(),
            unknown: Map::new(),
        }
    }

    #[test]
    fn invalid_request_does_not_mutate_database() {
        let database = path();
        let service = AssetService::open(&database).expect("open service");
        let mut invalid = request();
        invalid.name = " ".into();
        assert_eq!(
            service.create(invalid).unwrap_err().code,
            "VALIDATION_ERROR"
        );
        let valid = service.create(request()).expect("subsequent valid create");
        assert_eq!(valid.current_revision, 1);
        drop(service);
        fs::remove_file(&database).expect("remove exact temporary database");
    }
}
