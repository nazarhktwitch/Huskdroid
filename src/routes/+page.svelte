<script lang="ts">
  // Main app shell, sidebar + content area
  // Device list and navigation will be wired in as components are added
</script>

<div class="app-shell">
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="logo-text">Huskdroid</span>
    </div>
    <div class="sidebar-body">
      <p class="text-muted" style="padding: 16px; font-size: 12px;">
        No devices yet.
      </p>
    </div>
  </aside>

  <main class="main-content">
    <div class="empty-state animate-in">
      <h1>Huskdroid</h1>
      <p class="text-secondary">Android sandbox and development environment</p>
    </div>
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .sidebar {
    width: var(--sidebar-width);
    min-width: var(--sidebar-width);
    background: var(--bg-surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    height: 48px;
    padding: 0 16px;
    border-bottom: 1px solid var(--border);
  }

  .logo-text {
    font-weight: 600;
    font-size: 15px;
    color: var(--accent);
    letter-spacing: -0.02em;
  }

  .sidebar-body {
    flex: 1;
    overflow-y: auto;
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .empty-state {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .empty-state h1 {
    font-size: 24px;
    color: var(--text-primary);
  }
</style>
