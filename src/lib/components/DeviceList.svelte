<script lang="ts">
    import { onMount } from 'svelte';
    import { listDevices, type DeviceConfig } from '$lib/api/devices';
    import DeviceCard from './DeviceCard.svelte';

    let {
        selectedId = $bindable<string | null>(null),
        onAddClick
    }: {
        selectedId?: string | null;
        onAddClick?: () => void;
    } = $props();

    let devices = $state<DeviceConfig[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    onMount(async () => {
        try {
            devices = await listDevices();
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    });

    export async function refresh() {
        try {
            devices = await listDevices();
        } catch (e) {
            error = String(e);
        }
    }
</script>

<div class="device-list">
    <div class="list-header">
        <span class="list-label">Devices</span>
        <button class="add-btn" title="New device" onclick={onAddClick}>+</button>
    </div>

    {#if loading}
        <p class="list-hint">Loading...</p>
    {:else if error}
        <p class="list-hint text-red">{error}</p>
    {:else if devices.length === 0}
        <p class="list-hint">No devices. Click + to create one.</p>
    {:else}
        {#each devices as device (device.id)}
            <DeviceCard
                {device}
                selected={selectedId === device.id}
                onclick={() => (selectedId = device.id)}
            />
        {/each}
    {/if}
</div>

<style>
    .device-list {
        display: flex;
        flex-direction: column;
    }

    .list-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 8px 14px 4px;
    }

    .list-label {
        font-size: 11px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        color: var(--text-muted);
    }

    .add-btn {
        width: 20px;
        height: 20px;
        padding: 0;
        font-size: 18px;
        line-height: 1;
        color: var(--text-muted);
        background: transparent;
        border: none;
        border-radius: var(--radius-sm);
        cursor: pointer;
        transition: color var(--transition), background var(--transition);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .add-btn:hover {
        color: var(--text-primary);
        background: var(--bg-hover);
    }

    .list-hint {
        padding: 12px 14px;
        font-size: 12px;
        color: var(--text-muted);
        line-height: 1.5;
    }
</style>
