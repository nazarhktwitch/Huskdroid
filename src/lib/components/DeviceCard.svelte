<script lang="ts">
    import type { DeviceConfig } from '$lib/api/devices';

    let {
        device,
        selected = false,
        status = 'stopped',
        onclick
    }: {
        device: DeviceConfig;
        selected?: boolean;
        status?: 'running' | 'stopped' | 'error';
        onclick?: () => void;
    } = $props();
</script>

<button class="device-card" class:selected onclick={onclick}>
    <div class="device-info">
        <span class="device-name">{device.name}</span>
        <span class="device-meta">{device.android_version} &middot; {device.ram_mb} MB</span>
    </div>
    <span class="badge {status}">{status}</span>
</button>

<style>
    .device-card {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 10px 14px;
        background: transparent;
        border: none;
        border-radius: 0;
        border-left: 2px solid transparent;
        cursor: pointer;
        transition: background var(--transition), border-color var(--transition);
        text-align: left;
        gap: 8px;
    }

    .device-card:hover {
        background: var(--bg-hover);
    }

    .device-card.selected {
        background: var(--bg-active);
        border-left-color: var(--accent);
    }

    .device-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
        min-width: 0;
    }

    .device-name {
        font-size: 13px;
        font-weight: 500;
        color: var(--text-primary);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .device-meta {
        font-size: 11px;
        color: var(--text-muted);
        font-family: var(--font-mono);
    }
</style>
