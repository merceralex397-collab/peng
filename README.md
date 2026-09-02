# Peng

Peng is a local-first desktop library for prompts, skills, plugins, MCP server definitions, and subagents. It keeps one canonical copy of each asset and exports selected graphs as inspectable `.peng` bundles.

The repository is pre-release and currently at the initial Tauri 2, Svelte 5, TypeScript, Vite, and Rust scaffold.

## Development

Prerequisites: Node.js, npm, Rust, and the platform prerequisites required by Tauri 2.

```powershell
npm install
npm run check
npm run build
npm run tauri build -- --no-bundle
```

Product authority starts at [docs/product/vision.md](docs/product/vision.md). The supplied design source is retained in [`Peng_Design_Pack`](Peng_Design_Pack/).

