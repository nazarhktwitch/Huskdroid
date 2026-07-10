<script lang="ts">
    import type { DeviceConfig } from '$lib/api/devices';
    import { startDevice, stopDevice } from '$lib/api/runtime';
    import { deviceStatuses, setStatus, getStatus } from '$lib/stores/deviceStatus';

    let {
        device,
        selected = false,
        onclick
    }: {
        device: DeviceConfig;
        selected?: boolean;
        onclick?: () => void;
    } = $props();

    let busy = $state(false);

    async function handleStart(e: MouseEvent) {
        e.stopPropagation();
        busy = true;
        try {
            const result = await startDevice(device.id);
            setStatus(device.id, result.status);
        } catch (err) {
            setStatus(device.id, 'error');
            console.error(err);
        } finally {
            busy = false;
        }
    }

    async function handleStop(e: MouseEvent) {
        e.stopPropagation();
        busy = true;
        try {
            const result = await stopDevice(device.id);
            setStatus(device.id, result.status);
        } catch (err) {
            console.error(err);
        } finally {
            busy = false;
        }
    }
</script>

<div class="device-card" class:selected role="button" tabindex="0"
    onclick={onclick}
    onkeydown={(e) => e.key === 'Enter' && onclick?.()}
>
    <div class="device-info">
        <span class="device-name">{device.name}</span>
        <span class="device-meta mono">
            {device.android_version} &middot; {device.ram_mb} MB
            {#if device.root.enabled}<span class="text-yellow"> &middot; root</span>{/if}
        </span>
    </div>

    <div class="device-actions">
        {#if $deviceStatuses[device.id] === 'running'}
            <span class="badge running">running</span>
            <button class="ghost" disabled={busy} onclick={handleStop}>Stop</button>
        {:else}
            <span class="badge stopped">stopped</span>
            <button class="primary" disabled={busy} onclick={handleStart}>Start</button>
        {/if}
    </div>
</div>

<style>
    .device-card {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 10px 14px;
        background: transparent;
        border-left: 2px solid transparent;
        cursor: pointer;
        transition: background var(--transition), border-color var(--transition);
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
        flex: 1;
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
    }

    .device-actions {
        display: flex;
        align-items: center;
        gap: 6px;
        flex-shrink: 0;
    }

    .device-actions button {
        padding: 3px 8px;
        font-size: 11px;
    }
</style>
