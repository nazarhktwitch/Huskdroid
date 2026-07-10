<script lang="ts">
    let {
        deviceName,
        onConfirm,
        onCancel
    }: {
        deviceName: string;
        onConfirm: () => void;
        onCancel: () => void;
    } = $props();

    function onBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) onCancel();
    }

    function onKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') onCancel();
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onBackdropClick} onkeydown={onKeydown}>
    <div class="dialog animate-in" role="alertdialog" aria-modal="true">
        <div class="dialog-body">
            <p>Delete <strong>{deviceName}</strong>?</p>
            <p class="text-muted sub">This will remove the device configuration. The image file will not be deleted.</p>
        </div>
        <div class="dialog-footer">
            <button class="ghost" onclick={onCancel}>Cancel</button>
            <button class="danger" onclick={onConfirm}>Delete</button>
        </div>
    </div>
</div>

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
        width: 340px;
        box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
        overflow: hidden;
    }

    .dialog-body {
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .dialog-body p { font-size: 14px; }

    .sub { font-size: 12px; }

    .dialog-footer {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
        padding: 12px 20px;
        border-top: 1px solid var(--border);
        background: var(--bg-surface);
    }
</style>
