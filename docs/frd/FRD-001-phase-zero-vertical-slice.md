# FRD-001: Phase 0 vertical slice

## Behavior

Peng must prove one complete prompt lifecycle through the intended production boundaries: create, persist, search, export, delete, import, and recover.

## Acceptance criteria

1. A user can create and edit a prompt with stable UUID, name, summary, status, tags, body, and prompt-specific data.
2. Closing and reopening Peng retains the autosaved working draft.
3. Searching for a phrase from the prompt body returns and opens that prompt.
4. Export writes a valid deterministic Peng 1.0 archive containing the current prompt, embedded schema, and correct checksums.
5. Deleting the local prompt and importing its bundle recovers the same identity, content, and metadata.
6. A checksum mismatch, unsafe path, invalid schema document, or cancelled import changes no library data and extracts no destination files.
7. Import and preview never execute bundle content.

