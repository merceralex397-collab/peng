<script lang="ts">
  import { createAsset, deleteAsset, getAsset, updateAsset, type Asset, type PengError } from './lib/assets';

  const assetTypes = ['Prompts', 'Skills', 'Plugins', 'MCP servers', 'Subagents'];
  let name = '';
  let body = '';
  let active: Asset | null = null;
  let pending = false;
  let notice = 'Create a prompt to begin.';
  let error: PengError | null = null;

  function asPengError(value: unknown): PengError {
    if (typeof value === 'object' && value !== null && 'code' in value && 'message' in value) return value as PengError;
    return { code: 'UNEXPECTED_ERROR', message: value instanceof Error ? value.message : String(value), retryable: false };
  }

  async function run(action: () => Promise<void>) {
    if (pending) return;
    pending = true;
    error = null;
    try { await action(); }
    catch (reason) { error = asPengError(reason); notice = 'The operation did not complete.'; }
    finally { pending = false; }
  }

  function createPrompt() {
    return run(async () => {
      active = await createAsset({
        type: 'prompt', schemaVersion: '1.0', name, summary: '', status: 'draft', tags: [], body,
        typeData: {}, source: { kind: 'created' },
      });
      notice = `Created revision ${active.currentRevision}.`;
    });
  }

  function reloadPrompt() {
    if (!active) return;
    return run(async () => {
      active = await getAsset(active!.id);
      name = active.name;
      body = active.body;
      notice = `Reloaded revision ${active.currentRevision}.`;
    });
  }

  function updatePrompt() {
    if (!active) return;
    return run(async () => {
      active = await updateAsset({
        id: active!.id, expectedRevision: active!.currentRevision, type: active!.type,
        schemaVersion: active!.schemaVersion, name, summary: active!.summary, status: active!.status,
        tags: active!.tags, body, typeData: active!.typeData, source: active!.source,
      });
      notice = `Updated to revision ${active.currentRevision}.`;
    });
  }

  function removePrompt() {
    if (!active) return;
    return run(async () => {
      await deleteAsset({ id: active!.id, expectedRevision: active!.currentRevision });
      active = null;
      name = '';
      body = '';
      notice = 'Prompt deleted.';
    });
  }
</script>

<svelte:head><meta name="description" content="Peng local AI asset library" /></svelte:head>

<div class="shell">
  <header>
    <a class="brand" href="/" aria-label="Peng home">Peng <span>local library</span></a>
    <label class="search"><span class="sr-only">Search assets</span><input placeholder="Search arrives in the persistence phase" disabled /></label>
    <div class="actions"><button disabled>Import</button><button disabled>Export</button></div>
  </header>

  <main>
    <nav aria-label="Library">
      <p>Library</p>
      <a class="active" href="/">All items <span>{active ? 1 : 0}</span></a>
      <p>Types</p>
      {#each assetTypes as type}<a href="/">{type} <span>{type === 'Prompts' && active ? 1 : 0}</span></a>{/each}
    </nav>

    <section class="list" aria-labelledby="all-items-heading">
      <div class="section-heading"><h1 id="all-items-heading">Prompts</h1><span>{active ? '1 item' : '0 items'}</span></div>
      {#if active}
        <article class="asset-row" aria-current="true"><strong>{active.name}</strong><span>Revision {active.currentRevision}</span><code>{active.id}</code></article>
      {:else}
        <div class="empty"><div class="mark" aria-hidden="true">P</div><h2>Your library starts here</h2><p>Create a prompt in the workspace.</p></div>
      {/if}
    </section>

    <section class="workspace" aria-labelledby="editor-heading">
      <form on:submit|preventDefault={active ? updatePrompt : createPrompt}>
        <p class="eyebrow">Local SQLite prompt</p>
        <h2 id="editor-heading">{active ? 'Edit prompt' : 'Create prompt'}</h2>
        <label>Name<input bind:value={name} disabled={pending} autocomplete="off" /></label>
        <label>Body<textarea bind:value={body} disabled={pending} rows="12"></textarea></label>
        <div class="editor-actions">
          <button class="primary" type="submit" disabled={pending}>{pending ? 'Working…' : active ? 'Update' : 'Create'}</button>
          <button type="button" on:click={reloadPrompt} disabled={pending || !active}>Reload</button>
          <button class="danger" type="button" on:click={removePrompt} disabled={pending || !active}>Delete</button>
        </div>
        <p class="status" aria-live="polite">{notice}</p>
        {#if error}<div class="error" role="alert"><strong>{error.code}</strong><span>{error.message}</span>{#if error.field}<small>Field: {error.field}</small>{/if}</div>{/if}
      </form>
    </section>
  </main>
</div>
