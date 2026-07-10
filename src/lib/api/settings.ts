import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
    custom_qemu_path: string | null;
    custom_adb_path: string | null;
}

export async function getSettings(): Promise<AppSettings> {
    return invoke("get_settings");
}

export async function updateSettings(settings: AppSettings): Promise<void> {
    return invoke("update_settings", { settings });
}
