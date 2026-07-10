<script lang="ts">
    // Static list of known Android image sources. Without bundled images, download links only.
    interface RepoEntry {
        name: string;
        version: string;
        url: string;
        notes: string;
    }

    const entries: RepoEntry[] = [
        {
            name: "Android-x86",
            version: "9.0 (Pie)",
            url: "https://www.android-x86.org/download",
            notes: "x86 port, works with QEMU",
        },
        {
            name: "Android-x86",
            version: "7.1 (Nougat)",
            url: "https://www.android-x86.org/download",
            notes: "x86 port, works with QEMU",
        },
        {
            name: "LineageOS",
            version: "21 (Android 14)",
            url: "https://download.lineageos.org",
            notes: "Requires device-specific image",
        },
        {
            name: "LineageOS",
            version: "20 (Android 13)",
            url: "https://download.lineageos.org",
            notes: "Requires device-specific image",
        },
        {
            name: "Waydroid",
            version: "Android 13 (lineage-20)",
            url: "https://sourceforge.net/projects/waydroid/files/images/",
            notes: "Waydroid system + vendor images",
        },
        {
            name: "Waydroid",
            version: "Android 11 (lineage-18.1)",
            url: "https://sourceforge.net/projects/waydroid/files/images/",
            notes: "Waydroid system + vendor images",
        },
    ];
</script>

<div class="image-repo">
    <div class="section-header">
        <h2>Image Repository</h2>
        <p class="text-muted" style="font-size: 12px;">
            Images are not bundled. Download separately and import with the
            button above.
        </p>
    </div>

    <div class="repo-list">
        {#each entries as entry}
            <div class="repo-row">
                <div class="repo-info">
                    <span class="repo-name">{entry.name}</span>
                    <span class="repo-version text-secondary"
                        >{entry.version}</span
                    >
                    <span class="repo-notes text-muted">{entry.notes}</span>
                </div>
                <a
                    href={entry.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="download-link"
                >
                    Download
                </a>
            </div>
        {/each}
    </div>
</div>

<style>
    .image-repo {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .section-header {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .section-header h2 {
        font-size: 14px;
        font-weight: 600;
        color: var(--text-secondary);
        text-transform: uppercase;
        letter-spacing: 0.06em;
    }

    .repo-list {
        display: flex;
        flex-direction: column;
        gap: 1px;
        background: var(--border);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .repo-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 16px;
        background: var(--bg-surface);
        gap: 12px;
        transition: background var(--transition);
    }

    .repo-row:hover {
        background: var(--bg-hover);
    }

    .repo-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .repo-name {
        font-size: 13px;
        font-weight: 500;
    }

    .repo-version {
        font-size: 12px;
    }

    .repo-notes {
        font-size: 11px;
        font-family: var(--font-mono);
    }

    .download-link {
        font-size: 12px;
        font-weight: 500;
        color: var(--accent);
        text-decoration: none;
        padding: 4px 10px;
        border: 1px solid var(--accent-dim);
        border-radius: var(--radius-sm);
        white-space: nowrap;
        transition: background var(--transition);
    }

    .download-link:hover {
        background: var(--accent-glow);
    }
</style>
