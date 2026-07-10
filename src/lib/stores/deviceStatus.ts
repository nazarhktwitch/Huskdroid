import { writable } from 'svelte/store';

// Map of device ID -> status string
export const deviceStatuses = writable<Record<string, string>>({});

export function setStatus(id: string, status: string) {
    deviceStatuses.update((s) => ({ ...s, [id]: status }));
}

export function getStatus(statuses: Record<string, string>, id: string): string {
    return statuses[id] ?? 'stopped';
}
