<script lang="ts">
    // Main app shell, sidebar + content area
    import DeviceList from '$lib/components/DeviceList.svelte';
    import CreateDeviceDialog from '$lib/components/CreateDeviceDialog.svelte';
    import type { DeviceConfig } from '$lib/api/devices';

    let selectedId = $state<string | null>(null);
    let showCreateDialog = $state(false);
    let deviceList: DeviceList;

    function onDeviceCreated(device: DeviceConfig) {
        deviceList?.refresh();
        selectedId = device.id;
    }
</script>

<div class="app-shell">
    <aside class="sidebar">
        <div class="sidebar-header">
            <span class="logo-text">Huskdroid</span>
        </div>
        <div class="sidebar-body">
            <DeviceList
                bind:selectedId
                bind:this={deviceList}
                onAddClick={() => (showCreateDialog = true)}
            />
        </div>
    </aside>

    <main class="main-content">
        {#if selectedId}
            <div class="animate-in">
                <p class="text-secondary">Device selected: {selectedId}</p>
                <p class="text-muted" style="font-size: 12px; margin-top: 4px;">
                    Detail panel coming soon.
                </p>
            </div>
        {:else}
            <div class="empty-state animate-in">
                <h1>Huskdroid</h1>
                <p class="text-secondary">Select a device or create a new one.</p>
            </div>
        {/if}
    </main>
</div>

<CreateDeviceDialog bind:open={showCreateDialog} onCreated={onDeviceCreated} />
<style>
    .app-shell {
        display: flex;
        height: 100vh;
        overflow: hidden;
    }

    .sidebar {
        width: var(--sidebar-width);
        min-width: var(--sidebar-width);
        background: var(--bg-surface);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
    }

    .sidebar-header {
        display: flex;
        align-items: center;
        height: 48px;
        padding: 0 16px;
        border-bottom: 1px solid var(--border);
        flex-shrink: 0;
    }

    .logo-text {
        font-weight: 600;
        font-size: 15px;
        color: var(--accent);
        letter-spacing: -0.02em;
    }

    .sidebar-body {
        flex: 1;
        overflow-y: auto;
        padding-top: 8px;
    }

    .main-content {
        flex: 1;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .empty-state {
        text-align: center;
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .empty-state h1 {
        font-size: 24px;
        color: var(--text-primary);
    }
</style>
