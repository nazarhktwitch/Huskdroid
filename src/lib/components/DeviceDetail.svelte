<script lang="ts">
    import { onMount } from "svelte";
    import {
        getDevice,
        deleteDevice,
        type DeviceConfig,
    } from "$lib/api/devices";
    import { deviceStatuses, setStatus } from "$lib/stores/deviceStatus";
    import { startDevice, stopDevice, checkQemu } from "$lib/api/runtime";
    import SnapshotPanel from "./SnapshotPanel.svelte";
    import ConfirmDialog from "./ConfirmDialog.svelte";

    let { deviceId, onDeleted }: { deviceId: string; onDeleted?: () => void } =
        $props();

    let device = $state<DeviceConfig | null>(null);
    let loading = $state(true);
    let busy = $state(false);
    let error = $state<string | null>(null);
    let qemuOk = $state<boolean | null>(null);

    onMount(async () => {
        try {
            [device, qemuOk] = await Promise.all([
                getDevice(deviceId),
                checkQemu(),
            ]);
        } catch (e) {
            error = String(e);
        } finally {
            loading = false;
        }
    });

    async function handleStart() {
        busy = true;
        error = null;
        try {
            const r = await startDevice(deviceId);
            setStatus(deviceId, r.status);
        } catch (e) {
            error = String(e);
        } finally {
            busy = false;
        }
    }

    async function handleStop() {
        busy = true;
        try {
            const r = await stopDevice(deviceId);
            setStatus(deviceId, r.status);
        } catch (e) {
            error = String(e);
        } finally {
            busy = false;
        }
    }

    let showConfirm = $state(false);

    async function handleDelete() {
        try {
            await deleteDevice(deviceId);
            onDeleted?.();
        } catch (e) {
            error = String(e);
        } finally {
            showConfirm = false;
        }
    }

    const isRunning = $derived($deviceStatuses[deviceId] === "running");
</script>

<div class="device-detail animate-in">
    {#if loading}
        <p class="text-muted">Loading...</p>
    {:else if error}
        <p class="text-red">{error}</p>
    {:else if device}
        <div class="detail-header">
            <div class="detail-title">
                <h1>{device.name}</h1>
                <span class="badge {isRunning ? 'running' : 'stopped'}">
                    {isRunning ? "running" : "stopped"}
                </span>
            </div>
            <div class="detail-actions">
                {#if isRunning}
                    <button class="ghost" disabled={busy} onclick={handleStop}
                        >Stop</button
                    >
                {:else}
                    <button
                        class="primary"
                        disabled={busy || !qemuOk}
                        onclick={handleStart}
                    >
                        Start
                    </button>
                {/if}
                <button class="danger" onclick={() => (showConfirm = true)}
                    >Delete</button
                >
            </div>
        </div>

        {#if !qemuOk}
            <div class="warning-banner">
                QEMU not found in PATH - install QEMU to start devices.
            </div>
        {/if}

        {#if !device.image_path}
            <div class="warning-banner">
                No image assigned. Go to Images to import one.
            </div>
        {/if}

        <div class="info-grid">
            <div class="info-row">
                <span class="info-label">Android</span>
                <span class="info-val mono">{device.android_version}</span>
            </div>
            <div class="info-row">
                <span class="info-label">RAM</span>
                <span class="info-val mono">{device.ram_mb} MB</span>
            </div>
            <div class="info-row">
                <span class="info-label">Root</span>
                <span class="info-val mono"
                    >{device.root.enabled ? "enabled" : "disabled"}</span
                >
            </div>
            <div class="info-row">
                <span class="info-label">SELinux</span>
                <span class="info-val mono">{device.root.selinux}</span>
            </div>
            <div class="info-row">
                <span class="info-label">Network</span>
                <span class="info-val mono">
                    {device.sandbox.disable_network ? "isolated" : "enabled"}
                </span>
            </div>
            {#if device.image_path}
                <div class="info-row">
                    <span class="info-label">Image</span>
                    <span class="info-val mono truncate"
                        >{device.image_path}</span
                    >
                </div>
            {/if}
        </div>

        <SnapshotPanel deviceId={device.id} />
    {/if}
</div>

{#if showConfirm && device}
    <ConfirmDialog
        deviceName={device.name}
        onConfirm={handleDelete}
        onCancel={() => (showConfirm = false)}
    />
{/if}

<style>
    .device-detail {
        padding: 28px;
        height: 100%;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 20px;
        max-width: 640px;
    }

    .detail-header {
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        gap: 12px;
    }

    .detail-title {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .detail-title h1 {
        font-size: 20px;
        font-weight: 600;
    }

    .detail-actions {
        display: flex;
        gap: 8px;
        flex-shrink: 0;
    }

    .warning-banner {
        background: rgba(224, 168, 74, 0.1);
        border: 1px solid rgba(224, 168, 74, 0.3);
        border-radius: var(--radius);
        padding: 8px 14px;
        font-size: 12px;
        color: var(--yellow);
    }

    .info-grid {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .info-row {
        display: flex;
        align-items: center;
        padding: 8px 14px;
        gap: 16px;
        border-bottom: 1px solid var(--border);
    }

    .info-row:last-child {
        border-bottom: none;
    }

    .info-label {
        font-size: 12px;
        color: var(--text-muted);
        width: 80px;
        flex-shrink: 0;
    }

    .info-val {
        font-size: 12px;
        color: var(--text-secondary);
    }

    .truncate {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
