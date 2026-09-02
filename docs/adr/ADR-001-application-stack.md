# ADR-001: Tauri, Svelte, and Rust application stack

## Status

Accepted

## Decision

Use Tauri 2 as the desktop shell, Svelte 5 with TypeScript and Vite as a client-only SPA, and Rust for native commands and application services. Use the operating system webview rather than bundling Chromium.

## Consequences

- Native capabilities are granted per window through minimal Tauri permissions.
- Frontend code cannot issue arbitrary SQL or shell commands.
- Commands use typed requests, typed responses, and structured errors.
- CodeMirror language support is loaded only when an editor needs it.
- SvelteKit is not included without a concrete server/routing requirement.

