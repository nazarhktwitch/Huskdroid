<script lang="ts">
    import { createDevice, type DeviceConfig } from '$lib/api/devices';

    let {
        open = $bindable(false),
        onCreated
    }: {
        open?: boolean;
        onCreated?: (device: DeviceConfig) => void;
    } = $props();

    let name = $state('');
    let androidVersion = $state('Android 14');
    let ramMb = $state(2048);
    let cpuCores = $state(2);
    let rootEnabled = $state(false);
    let submitting = $state(false);
    let error = $state<string | null>(null);

    const ANDROID_VERSIONS = [
        'Android 4.4',
        'Android 7.1',
        'Android 8.1',
        'Android 9.0',
        'Android 10', 
        'Android 12', 
        'Android 13', 
        'Android 14', 
        'Android 15'
    ];
    const RAM_OPTIONS = [512, 1024, 2048, 4096, 8192];
    const CPU_OPTIONS = [1, 2, 4, 8, 16];

    async function submit() {
        if (!name.trim()) return;
        submitting = true;
        error = null;
        try {
            const device = await createDevice({
                name: name.trim(),
                android_version: androidVersion,
                ram_mb: ramMb,
                cpu_cores: cpuCores,
                root_enabled: rootEnabled,
            });
            onCreated?.(device);
            close();
        } catch (e) {
            error = String(e);
        } finally {
            submitting = false;
        }
    }

    function close() {
        open = false;
        name = '';
        androidVersion = 'Android 14';
        ramMb = 2048;
        cpuCores = 2;
        rootEnabled = false;
        error = null;
    }

    function onBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) close();
    }

    function onKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') close();
    }
</script>

{#if open}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={onBackdropClick} onkeydown={onKeydown}>
        <div class="dialog animate-in" role="dialog" aria-modal="true" aria-label="Create device">
            <div class="dialog-header">
                <h2>New Device</h2>
                <button class="close-btn" onclick={close} aria-label="Close">x</button>
            </div>

            <form class="dialog-body" onsubmit={(e) => { e.preventDefault(); submit(); }}>
                <label class="field">
                    <span class="field-label">Name</span>
                    <input
                        id="device-name"
                        type="text"
                        placeholder="e.g. Pixel_Test"
                        bind:value={name}
                        autocomplete="off"
                    />
                </label>

                <label class="field">
                    <span class="field-label">Android version</span>
                    <select id="device-android-version" bind:value={androidVersion}>
                        {#each ANDROID_VERSIONS as v}
                            <option value={v}>{v}</option>
                        {/each}
                    </select>
                </label>

                <label class="field">
                    <span class="field-label">RAM</span>
                    <select id="device-ram" bind:value={ramMb}>
                        {#each RAM_OPTIONS as r}
                            <option value={r}>{r} MB</option>
                        {/each}
                    </select>
                </label>

                <label class="field">
                    <span class="field-label">CPU Cores</span>
                    <select id="device-cpu" bind:value={cpuCores}>
                        {#each CPU_OPTIONS as c}
                            <option value={c}>{c} Cores</option>
                        {/each}
                    </select>
                </label>

                <label class="field field-row">
                    <span class="field-label">Enable root</span>
                    <input id="device-root" type="checkbox" bind:checked={rootEnabled} />
                </label>

                {#if error}
                    <p class="error-msg text-red">{error}</p>
                {/if}

                <div class="dialog-footer">
                    <button type="button" class="ghost" onclick={close}>Cancel</button>
                    <button
                        type="submit"
                        class="primary"
                        disabled={!name.trim() || submitting}
                    >
                        {submitting ? 'Creating...' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
    </div>
{/if}

<style>
    .backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
    }

    .dialog {
        background: var(--bg-elevated);
        border: 1px solid var(--border-strong);
        border-radius: var(--radius-lg);
        width: 380px;
        max-width: calc(100vw - 32px);
        box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
    }

    .dialog-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 16px 20px 12px;
        border-bottom: 1px solid var(--border);
    }

    .dialog-header h2 {
        font-size: 15px;
        font-weight: 600;
    }

    .close-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        font-size: 16px;
        padding: 2px 6px;
        cursor: pointer;
        border-radius: var(--radius-sm);
        transition: color var(--transition), background var(--transition);
    }

    .close-btn:hover {
        color: var(--text-primary);
        background: var(--bg-hover);
    }

    .dialog-body {
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .field-row {
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
    }

    .field-label {
        font-size: 12px;
        font-weight: 500;
        color: var(--text-secondary);
    }

    .error-msg {
        font-size: 12px;
    }

    .dialog-footer {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
        margin-top: 4px;
    }
</style>
