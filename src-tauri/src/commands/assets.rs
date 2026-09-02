use tauri::State;

use crate::application::AssetService;
use crate::domain::{Asset, CreateAssetRequest, DeleteAssetRequest, PengError, UpdateAssetRequest};

#[tauri::command]
pub fn create_asset(
    service: State<'_, AssetService>,
    request: CreateAssetRequest,
) -> Result<Asset, PengError> {
    service.create(request)
}

#[tauri::command]
pub fn get_asset(service: State<'_, AssetService>, id: String) -> Result<Asset, PengError> {
    service.get(&id)
}

#[tauri::command]
pub fn update_asset(
    service: State<'_, AssetService>,
    request: UpdateAssetRequest,
) -> Result<Asset, PengError> {
    service.update(request)
}

#[tauri::command]
pub fn delete_asset(
    service: State<'_, AssetService>,
    request: DeleteAssetRequest,
) -> Result<(), PengError> {
    service.delete(request)
}
