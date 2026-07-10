import { invoke } from '@tauri-apps/api/core';

export interface DeviceStatus {
    id: string;
    status: 'running' | 'stopped' | 'error';
    qemu_available: boolean;
}

export function startDevice(id: string): Promise<DeviceStatus> {
    return invoke('start_device', { id });
}

export function stopDevice(id: string): Promise<DeviceStatus> {
    return invoke('stop_device', { id });
}

export function deviceStatus(id: string): Promise<DeviceStatus> {
    return invoke('device_status', { id });
}

export function checkQemu(): Promise<boolean> {
    return invoke('check_qemu');
}
