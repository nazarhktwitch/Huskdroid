<script lang="ts">
    import { onMount } from 'svelte';
    import { open as openDialog } from '@tauri-apps/plugin-dialog';
    import { installApk, uninstallApk, listPackages, checkAdb } from '$lib/api/apk';

    let packages = $state<string[]>([]);
    let loading = $state(false);
    let adbAvailable = $state<boolean | null>(null);
    let installing = $state(false);
    let error = $state<string | null>(null);
    let dragging = $state(false);

    onMount(async () => {
        adbAvailable = await checkAdb();
        if (adbAvailable) await loadPackages();
    });

    async function loadPackages() {
        loading = true;
        try {
            packages = await listPackages();
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    }

    async function handleInstall() {
        const selected = await openDialog({
            title: 'Select APK',
            filters: [{ name: 'APK', extensions: ['apk'] }],
        });
        if (!selected) return;
        const path = typeof selected === 'string' ? selected : selected[0];
        await doInstall(path);
    }

    async function doInstall(path: string) {
        installing = true;
        error = null;
        try {
            await installApk(path);
            await loadPackages();
        } catch (e) {
            error = String(e);
        } finally {
            installing = false;
        }
    }

    async function handleUninstall(pkg: string) {
        try {
            await uninstallApk(pkg);
            packages = packages.filter((p) => p !== pkg);
        } catch (e) {
            error = String(e);
        }
    }

    function onDragOver(e: DragEvent) {
        e.preventDefault();
        dragging = true;
    }

    function onDragLeave() {
        dragging = false;
    }

    async function onDrop(e: DragEvent) {
        e.preventDefault();
        dragging = false;
        const file = e.dataTransfer?.files?.[0];
        if (file && file.name.endsWith('.apk')) {
            // Tauri file path from drag-drop
            await doInstall((file as any).path ?? file.name);
        }
    }
</script>

<div class="apk-manager">
    <div class="apk-header">
        <h2>APK Manager</h2>
        <button class="primary" onclick={handleInstall} disabled={installing || !adbAvailable}>
            {installing ? 'Installing...' : '+ Install APK'}
        </button>
    </div>

    {#if adbAvailable === false}
        <p class="msg text-yellow">adb not found in PATH. Install Android SDK platform-tools to use APK features.</p>
    {:else}
        <!-- Drop zone -->
        <div
            class="drop-zone"
            class:dragging
            ondragover={onDragOver}
            ondragleave={onDragLeave}
            ondrop={onDrop}
            role="region"
            aria-label="APK drop zone"
        >
            <span class="text-muted">Drop an APK here to install</span>
        </div>

        {#if error}
            <p class="msg text-red">{error}</p>
        {/if}

        {#if loading}
            <p class="msg text-muted">Loading packages...</p>
        {:else if packages.length === 0}
            <p class="msg text-muted">No packages installed, or device not connected.</p>
        {:else}
            <div class="pkg-list">
                {#each packages as pkg (pkg)}
                    <div class="pkg-row animate-in">
                        <span class="pkg-name mono">{pkg}</span>
                        <button class="danger" onclick={() => handleUninstall(pkg)}>Uninstall</button>
                    </div>
                {/each}
            </div>
        {/if}
    {/if}
</div>

<style>
    .apk-manager {
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 16px;
        height: 100%;
        overflow-y: auto;
    }

    .apk-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .apk-header h2 { font-size: 16px; }

    .msg { font-size: 13px; }

    .drop-zone {
        border: 1px dashed var(--border-strong);
        border-radius: var(--radius);
        padding: 20px;
        text-align: center;
        font-size: 13px;
        transition: border-color var(--transition), background var(--transition);
    }

    .drop-zone.dragging {
        border-color: var(--accent);
        background: var(--accent-glow);
    }

    .pkg-list {
        display: flex;
        flex-direction: column;
        gap: 1px;
        background: var(--border);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .pkg-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 9px 14px;
        background: var(--bg-surface);
        gap: 12px;
    }

    .pkg-row:hover { background: var(--bg-hover); }

    .pkg-name {
        font-size: 12px;
        color: var(--text-secondary);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
