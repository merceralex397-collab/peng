# Peng product vision

## Problem

Reusable AI prompts, skills, plugins, MCP definitions, and subagents are commonly scattered across tools and machine-specific folders. Copies drift, dependencies become unclear, and moving an exact working set between computers is difficult and risky.

## Vision

Peng is a small Windows-first, cross-platform desktop library that stores each reusable AI building block once, keeps it searchable and explainable, and projects it into different AI harnesses without turning generated copies into new sources of truth.

The product is local-first: core authoring, organisation, search, notes, history, import, and export work offline without an account or service.

## Product rule

**Store once, project many times.**

## Users and outcomes

Peng serves an individual developer or AI-tool power user who needs to:

- capture an asset quickly;
- find it without remembering a path;
- understand its purpose and history;
- compose assets by reference rather than copying content;
- inspect and transfer an exact set in one `.peng` file;
- keep secret values and execution outside ordinary content handling.

## V1 boundary

V1 is a local asset library and portable-bundle tool. It is not a chat client, cloud collaboration service, marketplace, Git host, general secret manager, background daemon, or automatic execution sandbox.

## Success

The first release is successful when all five asset types can be authored, searched, organised, versioned, related, safely imported/exported, backed up, restored, and operated entirely by keyboard as well as mouse.

