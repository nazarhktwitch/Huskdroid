import { invoke } from '@tauri-apps/api/core';

export interface RootConfig {
    enabled: boolean;
    selinux: string;
    magisk: boolean;
}

export interface SandboxConfig {
    disable_network: boolean;
    reset_on_shutdown: boolean;
    fake_device_info: boolean;
    fake_gps: boolean;
    isolated_storage: boolean;
}

export interface DeviceConfig {
    id: string;
    name: string;
    android_version: string;
    image_path: string | null;
    ram_mb: number;
    root: RootConfig;
    sandbox: SandboxConfig;
    created_at: string;
}

export interface CreateDevicePayload {
    name: string;
    android_version: string;
    ram_mb: number;
    root_enabled: boolean;
}

export function listDevices(): Promise<DeviceConfig[]> {
    return invoke('list_devices');
}

export function createDevice(payload: CreateDevicePayload): Promise<DeviceConfig> {
    return invoke('create_device', { payload });
}

export function deleteDevice(id: string): Promise<void> {
    return invoke('delete_device', { id });
}

export function getDevice(id: string): Promise<DeviceConfig | null> {
    return invoke('get_device', { id });
}

export function updateDevice(device: DeviceConfig): Promise<void> {
    return invoke('update_device', { device });
}
