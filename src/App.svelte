<script lang="ts">
  const assetTypes = ['Prompts', 'Skills', 'Plugins', 'MCP servers', 'Subagents'];
</script>

<svelte:head>
  <meta name="description" content="Peng local AI asset library" />
</svelte:head>

<div class="shell">
  <header>
    <a class="brand" href="/" aria-label="Peng home">Peng <span>local library</span></a>
    <label class="search">
      <span class="sr-only">Search assets</span>
      <input placeholder="Search prompts, skills, servers, notes…" disabled />
    </label>
    <div class="actions">
      <button disabled>Import</button>
      <button disabled>Export</button>
      <button class="primary" disabled>+ New</button>
    </div>
  </header>

  <main>
    <nav aria-label="Library">
      <p>Library</p>
      <a class="active" href="/">All items <span>0</span></a>
      <a href="/">Favourites <span>0</span></a>
      <a href="/">Recent <span>0</span></a>
      <a href="/">Archived <span>0</span></a>
      <p>Types</p>
      {#each assetTypes as type}
        <a href="/">{type} <span>0</span></a>
      {/each}
    </nav>

    <section class="list" aria-labelledby="all-items-heading">
      <div class="section-heading">
        <h1 id="all-items-heading">All items</h1>
        <span>0 items</span>
      </div>
      <div class="empty">
        <div class="mark" aria-hidden="true">P</div>
        <h2>Your library starts here</h2>
        <p>Create or import an asset to begin building your local AI toolkit.</p>
      </div>
    </section>

    <section class="workspace" aria-labelledby="welcome-heading">
      <div>
        <p class="eyebrow">Local-first · private · portable</p>
        <h2 id="welcome-heading">One canonical home for reusable AI building blocks.</h2>
        <p>Peng will keep prompts, skills, plugins, MCP definitions, and subagents searchable and ready to package as safe portable bundles.</p>
        <p class="status">Repository scaffold ready. Phase 0 implementation follows through Kanmer.</p>
      </div>
    </section>
  </main>
</div>

