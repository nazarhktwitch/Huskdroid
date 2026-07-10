import { invoke } from "@tauri-apps/api/core";

export interface DepsStatus {
    adb_found: boolean;
    adb_path: string;
    qemu_found: boolean;
    qemu_path: string;
}

export async function checkDeps(): Promise<DepsStatus> {
    return invoke("check_deps");
}

export async function downloadAdb(): Promise<void> {
    return invoke("download_adb");
}
