import { invoke } from '@tauri-apps/api/core';

export type ImageFormat = 'raw' | 'qcow2' | 'iso' | 'vdi' | 'vmdk' | 'folder' | 'unknown';

export interface ImageEntry {
    id: string;
    name: string;
    path: string;
    format: ImageFormat;
    added_at: string;
}

export function listImages(): Promise<ImageEntry[]> {
    return invoke('list_images');
}

export function importImage(name: string, path: string): Promise<ImageEntry> {
    return invoke('import_image', { name, path });
}

export function deleteImage(id: string): Promise<void> {
    return invoke('delete_image', { id });
}
