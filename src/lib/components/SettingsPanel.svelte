<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { getSettings, updateSettings, type AppSettings } from "$lib/api/settings";
    import { checkDeps, downloadAdb, type DepsStatus } from "$lib/api/deps";
    import { listen } from "@tauri-apps/api/event";
    import { open } from "@tauri-apps/plugin-dialog";

    let settings = $state<AppSettings | null>(null);
    let deps = $state<DepsStatus | null>(null);
    
    let loading = $state(true);
    let saving = $state(false);
    let adbDownloading = $state(false);
    let adbProgress = $state(0);
    let error = $state<string | null>(null);
    let unlisten: () => void;

    onMount(async () => {
        try {
            settings = await getSettings();
            deps = await checkDeps();
            
            const u = await listen<number>("adb_download_progress", (e) => {
                adbProgress = e.payload;
            });
            unlisten = u;
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    async function handleSaveSettings() {
        if (!settings) return;
        saving = true;
        error = null;
        try {
            await updateSettings(settings);
            // Re-check deps to see if the new qemu path is valid
            deps = await checkDeps();
        } catch (e) {
            error = String(e);
        } finally {
            saving = false;
        }
    }

    async function handleDownloadAdb() {
        adbDownloading = true;
        adbProgress = 0;
        error = null;
        try {
            await downloadAdb();
            deps = await checkDeps();
        } catch (e) {
            error = String(e);
        } finally {
            adbDownloading = false;
        }
    }

    async function handleBrowseQemu() {
        const selected = await open({
            multiple: false,
            filters: [{
                name: "Executable",
                extensions: ["exe", ""] // Empty string for linux binaries without extensions
            }]
        });
        if (selected && typeof selected === "string" && settings) {
            settings.custom_qemu_path = selected;
            handleSaveSettings();
        }
    }

    async function handleBrowseAdb() {
        const selected = await open({
            multiple: false,
            filters: [{
                name: "Executable",
                extensions: ["exe", ""] // Empty string for linux binaries without extensions
            }]
        });
        if (selected && typeof selected === "string" && settings) {
            settings.custom_adb_path = selected;
            handleSaveSettings();
        }
    }
</script>

<div class="settings-container animate-in">
    <div class="header">
        <h1>Settings</h1>
        <p>Global app configuration and dependencies.</p>
    </div>

    {#if error}
        <div class="error-banner">{error}</div>
    {/if}

    {#if loading}
        <div class="loading">Loading settings...</div>
    {:else if settings && deps}
        
        <div class="card">
            <h2>Dependencies</h2>
            <div class="dep-grid">
                <div class="dep-row">
                    <div class="dep-info">
                        <h3>Android Debug Bridge (ADB)</h3>
                        <p class="mono truncate path-text" title={deps.adb_path}>{deps.adb_found ? deps.adb_path : "Not found"}</p>
                    </div>
                    <div class="dep-action">
                        {#if deps.adb_found}
                            <span class="badge success">Installed</span>
                        {:else if adbDownloading}
                            <div class="progress-bar">
                                <div class="progress-fill" style="width: {adbProgress}%"></div>
                            </div>
                        {:else}
                            <button class="primary" onclick={handleDownloadAdb}>Download ADB</button>
                        {/if}
                    </div>
                </div>

                <div class="dep-row">
                    <div class="dep-info">
                        <h3>QEMU Emulator</h3>
                        <p class="mono truncate path-text" title={deps.qemu_path}>{deps.qemu_found ? deps.qemu_path : "Not found"}</p>
                    </div>
                    <div class="dep-action">
                        {#if deps.qemu_found}
                            <span class="badge success">Installed</span>
                        {:else}
                            <a class="button secondary" href="https://www.qemu.org/download/#windows" target="_blank">Download QEMU</a>
                        {/if}
                    </div>
                </div>
            </div>
        </div>

        <div class="card">
            <h2>Global Settings</h2>
            <div class="setting-row">
                <div class="setting-info">
                    <h3>Custom QEMU Path</h3>
                    <p>If QEMU is not in your PATH, provide the full path to <code>qemu-system-x86_64.exe</code> here.</p>
                </div>
                <div class="setting-input">
                    <div style="display: flex; gap: 8px;">
                        <input 
                            type="text" 
                            class="info-input mono" 
                            placeholder="C:\Program Files\qemu\qemu-system-x86_64.exe" 
                            bind:value={settings.custom_qemu_path}
                            onchange={handleSaveSettings}
                            disabled={saving}
                        />
                        <button class="secondary" onclick={handleBrowseQemu} disabled={saving}>Browse</button>
                    </div>
                </div>
            </div>

            <div class="setting-row" style="margin-top: 24px;">
                <div class="setting-info">
                    <h3>Custom ADB Path</h3>
                    <p>If you already have <code>adb.exe</code> installed elsewhere, provide the path here.</p>
                </div>
                <div class="setting-input">
                    <div style="display: flex; gap: 8px;">
                        <input 
                            type="text" 
                            class="info-input mono" 
                            placeholder="C:\platform-tools\adb.exe" 
                            bind:value={settings.custom_adb_path}
                            onchange={handleSaveSettings}
                            disabled={saving}
                        />
                        <button class="secondary" onclick={handleBrowseAdb} disabled={saving}>Browse</button>
                    </div>
                </div>
            </div>
        </div>
        
    {/if}
</div>

<style>
    .settings-container {
        padding: 40px;
        max-width: 800px;
        margin: 0 auto;
        height: 100%;
        overflow-y: auto;
    }

    .header {
        margin-bottom: 32px;
    }

    .header h1 {
        margin: 0 0 8px 0;
        font-size: 28px;
        font-weight: 600;
        color: var(--text-primary);
    }

    .header p {
        margin: 0;
        color: var(--text-secondary);
    }

    .card {
        background: var(--bg-secondary);
        border: 1px solid var(--border);
        border-radius: var(--radius-lg);
        padding: 24px;
        margin-bottom: 24px;
    }

    .card h2 {
        margin: 0 0 20px 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--text-primary);
        border-bottom: 1px solid var(--border);
        padding-bottom: 12px;
    }

    .dep-grid {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .dep-row, .setting-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 24px;
    }

    .dep-info, .setting-info {
        flex: 1;
        min-width: 0;
    }

    .dep-info h3, .setting-info h3 {
        margin: 0 0 4px 0;
        font-size: 15px;
        color: var(--text-primary);
    }

    .dep-info p, .setting-info p {
        margin: 0;
        font-size: 13px;
        color: var(--text-secondary);
    }

    .path-text {
        opacity: 0.7;
    }

    .dep-action {
        flex-shrink: 0;
        min-width: 140px;
        display: flex;
        justify-content: flex-end;
    }

    .setting-input {
        flex-shrink: 0;
        width: 300px;
    }

    .info-input {
        width: 100%;
        background: var(--bg-elevated);
        border: 1px solid var(--border);
        color: var(--text-primary);
        border-radius: var(--radius-sm);
        padding: 8px 12px;
        font-size: 13px;
    }

    .info-input:disabled {
        opacity: 0.5;
    }

    .badge {
        padding: 6px 12px;
        border-radius: var(--radius-full);
        font-size: 12px;
        font-weight: 500;
        background: rgba(46, 213, 115, 0.1);
        color: #2ed573;
        border: 1px solid rgba(46, 213, 115, 0.2);
    }

    .progress-bar {
        width: 100%;
        height: 12px;
        background: var(--bg-elevated);
        border-radius: var(--radius-full);
        overflow: hidden;
        border: 1px solid var(--border);
    }

    .progress-fill {
        height: 100%;
        background: var(--accent);
        transition: width 0.2s;
    }

    .button {
        display: inline-block;
        text-decoration: none;
        text-align: center;
    }
</style>
