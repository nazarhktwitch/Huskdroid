<script lang="ts">
    import { fsListDeviceDir, fsDeleteFile, type FsEntry } from '$lib/api/fs';

    let currentPath = $state('/sdcard');
    let entries = $state<string[]>([]);
    let loading = $state(false);
    let error = $state<string | null>(null);

    async function browse(path: string) {
        loading = true;
        error = null;
        currentPath = path;
        try {
            entries = await fsListDeviceDir(path);
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    }

    async function navigateUp() {
        const parent = currentPath.split('/').slice(0, -1).join('/') || '/';
        await browse(parent);
    }

    function parseEntryName(line: string): string {
        // ls -la output: permissions links user group size date time name
        const parts = line.trim().split(/\s+/);
        return parts.slice(8).join(' ') || line;
    }

    function isDir(line: string): boolean {
        return line.startsWith('d');
    }
</script>

<div class="fs-browser">
    <div class="fs-toolbar">
        <button class="ghost" onclick={navigateUp} disabled={currentPath === '/'}>↑ Up</button>
        <span class="path-display mono">{currentPath}</span>
        <button class="ghost" onclick={() => browse(currentPath)}>Refresh</button>
    </div>

    {#if !entries.length && !loading}
        <div class="fs-empty">
            <button class="primary" onclick={() => browse('/sdcard')}>Browse /sdcard</button>
            <p class="text-muted" style="font-size: 12px; margin-top: 8px;">
                Device must be running and connected via ADB.
            </p>
        </div>
    {:else if loading}
        <p class="fs-msg text-muted">Loading...</p>
    {:else if error}
        <p class="fs-msg text-red">{error}</p>
    {:else}
        <div class="entry-list">
            {#each entries as line (line)}
                {#if line && !line.startsWith('total')}
                    {@const name = parseEntryName(line)}
                    {@const dir = isDir(line)}
                    <div class="entry-row">
                        <span class="entry-icon">{dir ? '📁' : '📄'}</span>
                        <button
                            class="entry-name mono"
                            onclick={() => dir && browse(`${currentPath}/${name}`)}
                            disabled={!dir}
                        >{name}</button>
                        {#if !dir}
                            <button class="danger" onclick={() => fsDeleteFile(`${currentPath}/${name}`)}>
                                Delete
                            </button>
                        {/if}
                    </div>
                {/if}
            {/each}
        </div>
    {/if}
</div>

<style>
    .fs-browser {
        display: flex;
        flex-direction: column;
        gap: 8px;
        height: 100%;
        overflow: hidden;
    }

    .fs-toolbar {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 14px;
        background: var(--bg-surface);
        border-bottom: 1px solid var(--border);
        flex-shrink: 0;
    }

    .path-display {
        flex: 1;
        font-size: 12px;
        color: var(--text-secondary);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .fs-empty {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        flex: 1;
        gap: 8px;
    }

    .fs-msg {
        padding: 12px 16px;
        font-size: 12px;
    }

    .entry-list {
        flex: 1;
        overflow-y: auto;
    }

    .entry-row {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 6px 14px;
        border-bottom: 1px solid var(--border);
        transition: background var(--transition);
    }

    .entry-row:hover { background: var(--bg-hover); }

    .entry-icon { font-size: 14px; flex-shrink: 0; }

    .entry-name {
        flex: 1;
        font-size: 12px;
        color: var(--text-secondary);
        background: transparent;
        border: none;
        text-align: left;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        padding: 0;
        cursor: default;
    }

    .entry-name:not(:disabled) {
        color: var(--accent);
        cursor: pointer;
    }

    .entry-name:not(:disabled):hover { text-decoration: underline; }

    .entry-row button.danger {
        padding: 2px 6px;
        font-size: 11px;
        flex-shrink: 0;
    }
</style>
