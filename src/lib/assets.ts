import { invoke } from '@tauri-apps/api/core';

export type AssetStatus = 'draft' | 'ready' | 'deprecated' | 'archived';
export type SourceKind = 'created' | 'imported' | 'linked' | 'generated';

export interface AssetSource { kind: SourceKind; reference?: string; }

export interface Asset {
  id: string; type: string; schemaVersion: string; name: string; summary: string;
  status: AssetStatus; tags: string[]; body: string; typeData: Record<string, unknown>;
  source: AssetSource; currentRevision: number; createdAt: string; updatedAt: string;
  [key: string]: unknown;
}

export interface CreateAssetRequest {
  type: string; schemaVersion: string; name: string; summary: string; status: AssetStatus;
  tags: string[]; body: string; typeData: Record<string, unknown>; source: AssetSource;
  [key: string]: unknown;
}

export interface UpdateAssetRequest extends CreateAssetRequest { id: string; expectedRevision: number; }
export interface DeleteAssetRequest { id: string; expectedRevision: number; }
export interface PengError { code: string; message: string; field?: string; details?: unknown; retryable: boolean; }

export function createAsset(request: CreateAssetRequest): Promise<Asset> {
  return invoke<Asset>('create_asset', { request });
}
export function getAsset(id: string): Promise<Asset> { return invoke<Asset>('get_asset', { id }); }
export function updateAsset(request: UpdateAssetRequest): Promise<Asset> {
  return invoke<Asset>('update_asset', { request });
}
export function deleteAsset(request: DeleteAssetRequest): Promise<void> {
  return invoke<void>('delete_asset', { request });
}
