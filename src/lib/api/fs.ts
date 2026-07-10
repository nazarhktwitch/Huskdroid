import { invoke } from '@tauri-apps/api/core';

export interface FsEntry {
    name: string;
    path: string;
    is_dir: boolean;
    size: number | null;
}

export function fsListDir(path: string): Promise<FsEntry[]> {
    return invoke('fs_list_dir', { path });
}

export function fsListDeviceDir(remote: string): Promise<string[]> {
    return invoke('fs_list_device_dir', { remote });
}

export function fsPullFile(remote: string, local: string): Promise<void> {
    return invoke('fs_pull_file', { remote, local });
}

export function fsPushFile(local: string, remote: string): Promise<void> {
    return invoke('fs_push_file', { local, remote });
}

export function fsDeleteFile(remote: string): Promise<void> {
    return invoke('fs_delete_file', { remote });
}
