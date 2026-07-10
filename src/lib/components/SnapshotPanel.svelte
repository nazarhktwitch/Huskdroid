<script lang="ts">
    import { onMount } from 'svelte';
    import {
        listSnapshots,
        createSnapshot,
        restoreSnapshot,
        deleteSnapshot,
        type Snapshot,
    } from '$lib/api/snapshots';

    let { deviceId }: { deviceId: string } = $props();

    let snapshots = $state<Snapshot[]>([]);
    let loading = $state(true);
    let creating = $state(false);
    let newName = $state('');
    let error = $state<string | null>(null);

    onMount(() => load());

    async function load() {
        loading = true;
        try {
            snapshots = await listSnapshots(deviceId);
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    }

    async function handleCreate() {
        const name = newName.trim() || `Snapshot ${new Date().toLocaleString()}`;
        creating = true;
        error = null;
        try {
            const snap = await createSnapshot(deviceId, name);
            snapshots = [...snapshots, snap];
            newName = '';
        } catch (e) {
            error = String(e);
        } finally {
            creating = false;
        }
    }

    async function handleRestore(id: string) {
        error = null;
        try {
            await restoreSnapshot(id, deviceId);
        } catch (e) {
            error = String(e);
        }
    }

    async function handleDelete(id: string) {
        try {
            await deleteSnapshot(id);
            snapshots = snapshots.filter((s) => s.id !== id);
        } catch (e) {
            error = String(e);
        }
    }

    function formatSize(bytes: number): string {
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
        return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
    }

    function formatDate(iso: string): string {
        return new Date(iso).toLocaleString();
    }
</script>

<div class="snapshot-panel">
    <div class="snap-header">
        <h3>Snapshots</h3>
    </div>

    <div class="snap-create">
        <input
            id="snapshot-name"
            type="text"
            placeholder="Snapshot name (optional)"
            bind:value={newName}
            class="mono"
        />
        <button class="primary" onclick={handleCreate} disabled={creating}>
            {creating ? 'Saving...' : 'Take snapshot'}
        </button>
    </div>

    {#if error}
        <p class="snap-msg text-red">{error}</p>
    {/if}

    {#if loading}
        <p class="snap-msg text-muted">Loading...</p>
    {:else if snapshots.length === 0}
        <p class="snap-msg text-muted">No snapshots yet.</p>
    {:else}
        <div class="snap-list">
            {#each snapshots as snap (snap.id)}
                <div class="snap-row animate-in">
                    <div class="snap-info">
                        <span class="snap-name">{snap.name}</span>
                        <span class="snap-meta mono">
                            {formatDate(snap.created_at)} &middot; {formatSize(snap.size_bytes)}
                        </span>
                    </div>
                    <div class="snap-actions">
                        <button class="ghost" onclick={() => handleRestore(snap.id)}>Restore</button>
                        <button class="danger" onclick={() => handleDelete(snap.id)}>Delete</button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .snapshot-panel {
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding-top: 20px;
        border-top: 1px solid var(--border);
    }

    .snap-header h3 {
        font-size: 13px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.06em;
        color: var(--text-muted);
    }

    .snap-create {
        display: flex;
        gap: 8px;
    }

    .snap-create input { flex: 1; }

    .snap-msg { font-size: 12px; }

    .snap-list {
        display: flex;
        flex-direction: column;
        gap: 1px;
        background: var(--border);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .snap-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 14px;
        background: var(--bg-surface);
        gap: 12px;
    }

    .snap-row:hover { background: var(--bg-hover); }

    .snap-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
        min-width: 0;
    }

    .snap-name {
        font-size: 13px;
        font-weight: 500;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .snap-meta {
        font-size: 11px;
        color: var(--text-muted);
    }

    .snap-actions {
        display: flex;
        gap: 6px;
        flex-shrink: 0;
    }

    .snap-actions button {
        padding: 3px 8px;
        font-size: 11px;
    }
</style>
