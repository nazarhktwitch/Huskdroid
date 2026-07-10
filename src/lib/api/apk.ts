import { invoke } from '@tauri-apps/api/core';

export function installApk(path: string): Promise<string> {
    return invoke('install_apk', { path });
}

export function uninstallApk(pkg: string): Promise<string> {
    return invoke('uninstall_apk', { package: pkg });
}

export function listPackages(): Promise<string[]> {
    return invoke('list_packages');
}

export function execShell(cmd: string): Promise<string> {
    return invoke('exec_shell', { cmd });
}

export function checkAdb(): Promise<boolean> {
    return invoke('check_adb');
}
