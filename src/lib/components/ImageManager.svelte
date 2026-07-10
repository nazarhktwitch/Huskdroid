<script lang="ts">
    import { onMount } from 'svelte';
    import { listImages, importImage, deleteImage, type ImageEntry } from '$lib/api/images';

    let images = $state<ImageEntry[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);
    let importing = $state(false);

    onMount(async () => {
        await load();
    });

    async function load() {
        try {
            images = await listImages();
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    }

    async function handleImport() {
        // Use Tauri's dialog plugin to pick a file or folder
        const { open: openDialog } = await import('@tauri-apps/plugin-dialog');
        const selected = await openDialog({
            title: 'Select Android image',
            multiple: false,
            directory: false,
            filters: [
                { name: 'Android images', extensions: ['img', 'raw', 'qcow2', 'iso', 'vdi', 'vmdk'] },
                { name: 'All files', extensions: ['*'] },
            ],
        });
        if (!selected) return;

        const path = typeof selected === 'string' ? selected : selected[0];
        const name = path.split(/[\\/]/).pop() ?? path;

        importing = true;
        try {
            await importImage(name, path);
            await load();
        } catch (e) {
            error = String(e);
        } finally {
            importing = false;
        }
    }

    async function handleDelete(id: string) {
        try {
            await deleteImage(id);
            images = images.filter((i) => i.id !== id);
        } catch (e) {
            error = String(e);
        }
    }

    function formatLabel(fmt: string): string {
        const labels: Record<string, string> = {
            raw: 'RAW',
            qcow2: 'QCOW2',
            iso: 'ISO',
            vdi: 'VDI',
            vmdk: 'VMDK',
            folder: 'Folder',
            unknown: 'Unknown',
        };
        return labels[fmt] ?? fmt;
    }
</script>

<div class="image-manager">
    <div class="section-header">
        <h2>Images</h2>
        <button class="primary" onclick={handleImport} disabled={importing}>
            {importing ? 'Importing...' : '+ Import'}
        </button>
    </div>

    {#if error}
        <p class="msg text-red">{error}</p>
    {/if}

    {#if loading}
        <p class="msg text-muted">Loading...</p>
    {:else if images.length === 0}
        <p class="msg text-muted">No images imported. Use "Import" to add an image file or folder.</p>
    {:else}
        <div class="image-list">
            {#each images as img (img.id)}
                <div class="image-row animate-in">
                    <div class="image-info">
                        <span class="image-name">{img.name}</span>
                        <span class="image-meta mono">{formatLabel(img.format)} &middot; {img.path}</span>
                    </div>
                    <button class="danger" onclick={() => handleDelete(img.id)}>Remove</button>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .image-manager {
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .section-header h2 {
        font-size: 16px;
    }

    .msg {
        font-size: 13px;
    }

    .image-list {
        display: flex;
        flex-direction: column;
        gap: 1px;
        background: var(--border);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .image-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 16px;
        background: var(--bg-surface);
        gap: 12px;
    }

    .image-row:hover {
        background: var(--bg-hover);
    }

    .image-info {
        display: flex;
        flex-direction: column;
        gap: 3px;
        min-width: 0;
    }

    .image-name {
        font-size: 13px;
        font-weight: 500;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .image-meta {
        font-size: 11px;
        color: var(--text-muted);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>
