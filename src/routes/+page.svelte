<script lang="ts">
    import DeviceList from '$lib/components/DeviceList.svelte';
    import CreateDeviceDialog from '$lib/components/CreateDeviceDialog.svelte';
    import DeviceDetail from '$lib/components/DeviceDetail.svelte';
    import ImageManager from '$lib/components/ImageManager.svelte';
    import ImageRepo from '$lib/components/ImageRepo.svelte';
    import ApkManager from '$lib/components/ApkManager.svelte';
    import AdbConsole from '$lib/components/AdbConsole.svelte';
    import type { DeviceConfig } from '$lib/api/devices';

    type NavView = 'devices' | 'images' | 'apk';

    let view = $state<NavView>('devices');
    let selectedId = $state<string | null>(null);
    let showCreateDialog = $state(false);
    let showAdb = $state(false);
    let deviceList = $state<DeviceList>();

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

        <nav class="sidebar-nav">
            <button class="nav-item" class:active={view === 'devices'} onclick={() => (view = 'devices')}>
                Devices
            </button>
            <button class="nav-item" class:active={view === 'images'} onclick={() => (view = 'images')}>
                Images
            </button>
            <button class="nav-item" class:active={view === 'apk'} onclick={() => (view = 'apk')}>
                APK
            </button>
        </nav>

        {#if view === 'devices'}
            <div class="sidebar-body">
                <DeviceList
                    bind:selectedId
                    bind:this={deviceList}
                    onAddClick={() => (showCreateDialog = true)}
                />
            </div>
        {/if}
    </aside>

    <main class="main-content">
        {#if view === 'images'}
            <div class="panel-view">
                <ImageManager />
                <div class="divider"></div>
                <ImageRepo />
            </div>
        {:else if view === 'apk'}
            <div class="panel-view split-v">
                <div class="split-content">
                    <ApkManager />
                </div>
                <div class="adb-toggle-bar">
                    <button class="ghost" onclick={() => (showAdb = !showAdb)}>
                        {showAdb ? 'Hide ADB Console' : 'Show ADB Console'}
                    </button>
                </div>
                <div class="adb-pane" class:visible={showAdb}>
                    <AdbConsole />
                </div>
            </div>
        {:else if selectedId}
            <DeviceDetail
                deviceId={selectedId}
                onDeleted={() => { selectedId = null; deviceList?.refresh(); }}
            />
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

    .sidebar-nav {
        display: flex;
        gap: 2px;
        padding: 4px;
        margin: 12px 12px 0;
        background: var(--bg-surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        flex-shrink: 0;
    }

    .nav-item {
        flex: 1;
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 6px 4px;
        font-size: 11px;
        font-weight: 500;
        color: var(--text-muted);
        background: transparent;
        border: none;
        border-radius: var(--radius-sm);
        cursor: pointer;
        text-align: center;
        transition: all var(--transition);
    }

    .nav-item:hover {
        background: var(--bg-hover);
        color: var(--text-primary);
    }

    .nav-item.active {
        background: var(--bg-elevated);
        color: var(--text-primary);
        box-shadow: 0 1px 3px rgba(0,0,0,0.2);
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
        align-items: stretch;
    }

    .panel-view {
        flex: 1;
        overflow-y: auto;
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 0;
    }

    .panel-view.split-v {
        padding: 0;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .split-content {
        flex: 1;
        overflow-y: auto;
    }

    .adb-pane {
        display: none;
        height: 280px;
        flex-shrink: 0;
    }

    .adb-pane.visible {
        display: flex;
        flex-direction: column;
    }

    .adb-toggle-bar {
        display: flex;
        justify-content: flex-end;
        padding: 6px 12px;
        background: var(--bg-surface);
        border-top: 1px solid var(--border);
        border-bottom: 1px solid var(--border);
    }

    .adb-toggle-bar button {
        font-size: 11px;
        padding: 4px 10px;
    }

    .empty-state {
        display: flex;
        flex-direction: column;
        gap: 8px;
        text-align: center;
        margin: auto;
    }

</style>
