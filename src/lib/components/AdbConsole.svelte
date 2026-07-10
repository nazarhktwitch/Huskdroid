<script lang="ts">
    import { execShell } from '$lib/api/apk';

    let input = $state('');
    let history = $state<{ cmd: string; output: string; error: boolean }[]>([]);
    let running = $state(false);
    let consoleEl: HTMLDivElement;

    async function submit(e: SubmitEvent) {
        e.preventDefault();
        const cmd = input.trim();
        if (!cmd) return;

        input = '';
        running = true;
        try {
            const output = await execShell(cmd);
            history = [...history, { cmd, output: output || '(no output)', error: false }];
        } catch (err) {
            history = [...history, { cmd, output: String(err), error: true }];
        } finally {
            running = false;
            // Scroll to bottom after render
            setTimeout(() => {
                consoleEl?.scrollTo({ top: consoleEl.scrollHeight, behavior: 'smooth' });
            }, 10);
        }
    }

    function clear() {
        history = [];
    }
</script>

<div class="adb-console">
    <div class="console-header">
        <span class="console-title">ADB Shell</span>
        <button class="ghost" onclick={clear}>Clear</button>
    </div>

    <div class="console-output" bind:this={consoleEl}>
        {#if history.length === 0}
            <p class="console-hint text-muted">Run adb shell commands. Device must be connected via ADB.</p>
        {/if}
        {#each history as entry}
            <div class="console-entry">
                <span class="console-prompt">$ {entry.cmd}</span>
                <pre class="console-result" class:error={entry.error}>{entry.output}</pre>
            </div>
        {/each}
        {#if running}
            <p class="console-hint text-muted">Running...</p>
        {/if}
    </div>

    <form class="console-input-row" onsubmit={submit}>
        <span class="prompt-symbol">$</span>
        <input
            id="adb-shell-input"
            type="text"
            placeholder="adb shell command..."
            bind:value={input}
            disabled={running}
            autocomplete="off"
            spellcheck="false"
            class="mono"
        />
        <button type="submit" class="primary" disabled={running || !input.trim()}>Run</button>
    </form>
</div>

<style>
    .adb-console {
        display: flex;
        flex-direction: column;
        height: 100%;
        background: var(--bg-base);
        border-top: 1px solid var(--border);
    }

    .console-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 8px 16px;
        border-bottom: 1px solid var(--border);
        background: var(--bg-surface);
        flex-shrink: 0;
    }

    .console-title {
        font-size: 12px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.06em;
        color: var(--text-muted);
    }

    .console-output {
        flex: 1;
        overflow-y: auto;
        padding: 12px 16px;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .console-hint {
        font-size: 12px;
        font-family: var(--font-mono);
    }

    .console-entry {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .console-prompt {
        font-family: var(--font-mono);
        font-size: 12px;
        color: var(--accent);
    }

    .console-result {
        font-family: var(--font-mono);
        font-size: 12px;
        color: var(--text-secondary);
        white-space: pre-wrap;
        word-break: break-all;
        margin: 0;
        padding: 6px 10px;
        background: var(--bg-elevated);
        border-radius: var(--radius-sm);
        border-left: 2px solid var(--border);
        user-select: text;
    }

    .console-result.error {
        color: var(--red);
        border-left-color: var(--red);
    }

    .console-input-row {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 10px 16px;
        border-top: 1px solid var(--border);
        background: var(--bg-surface);
        flex-shrink: 0;
    }

    .prompt-symbol {
        font-family: var(--font-mono);
        font-size: 13px;
        color: var(--accent);
        flex-shrink: 0;
    }

    .console-input-row input {
        flex: 1;
        background: var(--bg-elevated);
    }
</style>
