import { invoke } from '@tauri-apps/api/core';

export interface Snapshot {
    id: string;
    device_id: string;
    name: string;
    path: string;
    created_at: string;
    size_bytes: number;
}

export function listSnapshots(deviceId: string): Promise<Snapshot[]> {
    return invoke('list_snapshots', { deviceId });
}

export function createSnapshot(deviceId: string, name: string): Promise<Snapshot> {
    return invoke('create_snapshot', { deviceId, name });
}

export function restoreSnapshot(snapshotId: string, deviceId: string): Promise<void> {
    return invoke('restore_snapshot', { snapshotId, deviceId });
}

export function deleteSnapshot(snapshotId: string): Promise<void> {
    return invoke('delete_snapshot', { snapshotId });
}
